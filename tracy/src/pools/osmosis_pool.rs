use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use async_trait::async_trait;
use eyre::Result;

use crate::util::denom_trace::{
    load_denom_trace_cache_from_file, resolve_ibc, save_denom_trace_cache_to_file,
};
use crate::util::proto::osmosis_gamm_v1beta1::query_client::QueryClient;
use crate::util::proto::osmosis_gamm_v1beta1::{QuerySwapExactAmountInRequest, SwapAmountInRoute};
use crate::{Pool, PoolConfig, Quote};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OsmosisPool {
    #[serde(alias = "address")]
    pub pool_address: String,
    pub id: String,
    #[serde(alias = "poolParams")]
    pub pool_params: OsmosisPoolParams,
    pub future_pool_governor: String,
    #[serde(alias = "totalShares")]
    pub total_shares: OsmosisPoolToken,
    #[serde(alias = "poolAssets")]
    pub pool_assets: Vec<OsmosisPoolAssets>,
    #[serde(alias = "totalWeight")]
    pub total_weight: String,
    pub chain: Option<String>,
}

#[derive(Debug)]
pub enum TracyError {
    Only2AssersError,
}

impl std::error::Error for TracyError {}
unsafe impl Sync for TracyError {}
unsafe impl Send for TracyError {}

impl fmt::Display for TracyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TracyError::Only2AssersError => write!(
                f,
                "Can only calculate quote for pools with 2 assets, estimate quote instead."
            ),
        }
    }
}

impl OsmosisPool {
    // takes ibc or native denom and converts to correct type
    fn asset_for_denom(&self, denom: &str) -> Option<usize> {
        self.pool_assets.iter().position(|x| {
            x.token.denom == denom
                || match x.token.native_name.clone() {
                    Some(x) => x == denom,
                    None => false,
                }
        })
    }

    fn calculate_quote(
        &self,
        amount: u128,
        token_in_amount: u128,
        token_out_amount: u128,
        token_in_weight: u128,
        token_out_weight: u128,
        token_in_decimals: u32,
        token_out_decimals: u32,
    ) -> Result<u128> {
        if self.pool_assets.len() != 2 {
            return Err(TracyError::Only2AssersError.into());
        }
        // only on block by block basis, no time weighted average
        Ok(
            (token_out_amount * token_out_weight * u128::from(token_out_decimals) * amount)
                / (token_in_amount * token_in_weight * u128::from(token_in_decimals)),
        )
    }

    // TODO: gRPC parameter
    async fn estimate_quote(
        &self,
        amount: u128,
        token_in_denom: &str,
        token_out_denom: &str,
        config: &PoolConfig,
    ) -> Result<u128> {
        let mut client = QueryClient::connect(config.grpc_url.clone().unwrap()).await?;

        let pool_id = u64::from_str_radix(&self.id, 10)?;
        let token_in_index = self.asset_for_denom(token_in_denom).unwrap();

        let token_out_index = self.asset_for_denom(token_out_denom).unwrap();
        let request = QuerySwapExactAmountInRequest {
            sender: self.pool_address.clone(), // small hack because it uses SwapExactAmountIn just without writing new state so we need a address with enought liquidity, we assume the pool has that
            pool_id: pool_id,
            token_in: format!("{}{}", amount, self.pool_assets[token_in_index].token.denom),
            routes: vec![SwapAmountInRoute {
                pool_id: pool_id,
                token_out_denom: self.pool_assets[token_out_index].token.denom.clone(),
            }],
        };
        let response = client.estimate_swap_exact_amount_in(request).await?;

        Ok(u128::from_str_radix(
            &response.into_inner().token_out_amount,
            10,
        )?)
    }
}

#[async_trait]
impl Pool for OsmosisPool {
    async fn get_quote(
        &self,
        amount: u128,
        token_in_denom: &str,
        token_out_denom: &str,
        config: &PoolConfig,
    ) -> Result<Quote> {
        if config.estimate_quote {
            Ok(Quote {
                token_in: Some(amount),
                token_out: Some(
                    self.estimate_quote(amount, token_in_denom, token_out_denom, config)
                        .await?,
                ),
                pool_address: Some(self.pool_address.clone()),
                error: None,
            })
        } else {
            let token_in_index = self.asset_for_denom(token_in_denom).unwrap();
            let token_out_index = self.asset_for_denom(token_out_denom).unwrap();

            let token_in_amount =
                u128::from_str_radix(&self.pool_assets[token_in_index].token.amount, 10)?;
            let token_out_amount =
                u128::from_str_radix(&self.pool_assets[token_out_index].token.amount, 10)?;
            let token_in_weight =
                u128::from_str_radix(&self.pool_assets[token_in_index].weight, 10)?;
            let token_out_weight =
                u128::from_str_radix(&self.pool_assets[token_out_index].weight, 10)?;
            let token_in_decimals = 6;
            let token_out_decimals = 6;

            Ok(Quote {
                token_in: Some(amount),
                token_out: Some(self.calculate_quote(
                    amount,
                    token_in_amount,
                    token_out_amount,
                    token_in_weight,
                    token_out_weight,
                    token_in_decimals,
                    token_out_decimals,
                )?),
                pool_address: Some(self.pool_address.clone()),
                error: None,
            })
        }
    }
    fn token_denoms(&self) -> Vec<String> {
        let mut denoms: Vec<String> = self
            .pool_assets
            .iter()
            .map(|x| x.token.denom.clone())
            .collect();
        let native_denoms: Vec<String> = self
            .pool_assets
            .iter()
            .map(|x| x.token.native_name.clone())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        denoms.extend(native_denoms);

        denoms
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn chain(&self) -> String {
        String::from("osmosis")
    }

    fn address(&self) -> Result<String> {
        Ok(self.pool_address.clone())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OsmosisPoolParams {
    #[serde(alias = "swapFee")]
    swap_fee: String,
    #[serde(alias = "exitFee")]
    exit_fee: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OsmosisPoolToken {
    pub denom: String,
    pub amount: String,
    pub native_name: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct OsmosisPoolAssets {
    pub token: OsmosisPoolToken,
    pub weight: String,
}

#[derive(Debug, serde::Deserialize)]
struct OsmosisPoolsFetchResult {
    pools: Vec<OsmosisPool>,
}

pub async fn fetch_osmosis_pools(lcd_api: &str) -> Result<()> {
    // TODO: currently only ~800 pools, may need to use pagination
    let resp: OsmosisPoolsFetchResult = reqwest::get(format!(
        "{}/osmosis/gamm/v1beta1/pools?pagination.limit=1000",
        lcd_api
    ))
    .await?
    .json()
    .await?;

    let pools_raw = resp.pools;
    // TODO: can we not copy here?
    let mut pools: Vec<OsmosisPool> = vec![];
    let mut ibc_cache = load_denom_trace_cache_from_file(Path::new("denom_traces.json"))?;

    // #TODO: this loop is parallelizable ~~but that makes no sense at this time because the api server would rate limit us~~
    // nevermind, this is already pretty fast using the cache
    for pool in pools_raw {
        // TODO: this should probably be mapable
        let mut assets: Vec<OsmosisPoolAssets> = vec![];
        for asset in pool.pool_assets {
            let (native_denom, cache) =
                resolve_ibc(ibc_cache, lcd_api, asset.token.denom.clone(), true).await?;
            ibc_cache = cache;
            assets.push(OsmosisPoolAssets {
                token: OsmosisPoolToken {
                    denom: asset.token.denom,
                    amount: asset.token.amount,
                    native_name: native_denom,
                },
                weight: asset.weight,
            })
        }

        pools.push(OsmosisPool {
            pool_address: pool.pool_address,
            id: pool.id,
            pool_params: pool.pool_params,
            future_pool_governor: pool.future_pool_governor,
            total_shares: pool.total_shares,
            pool_assets: assets,
            total_weight: pool.total_weight,
            chain: Some("osmosis".to_owned()),
        })
    }

    let save_result = save_denom_trace_cache_to_file(Path::new("denom_traces.json"), ibc_cache);
    if let Err(x) = save_result {
        println!("could not save trace cache file error: {}", x);
    }
    let text = serde_json::to_string(&pools)?;
    let path = Path::new("osmosis_pools_hackathon.json");
    let mut file = File::create(path)?;
    file.write(text.as_bytes())?;

    Ok(())
}

// TODO: move fetch + load to trait
pub fn load_osmo_pools_from_file_boxed(path: &Path) -> Result<Vec<Box<OsmosisPool>>> {
    let mut file = File::open(path)?;

    let mut text: String = "".to_string();
    file.read_to_string(&mut text)?;
    let pools: Vec<Box<OsmosisPool>> = serde_json::from_str(&text)?;
    Ok(pools)
}
