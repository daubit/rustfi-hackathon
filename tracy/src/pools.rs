use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

use async_trait::async_trait;
use eyre::Result;
use tokio::time::sleep;

use crate::util::denom_trace::denom_trace;
use crate::util::proto::osmosis_gamm_v1beta1::query_client::QueryClient;
use crate::util::proto::osmosis_gamm_v1beta1::{QuerySwapExactAmountInRequest, SwapAmountInRoute};
use crate::{Pool, Quote};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OsmosisPool {
    address: String,
    id: String,
    #[serde(alias = "poolParams")]
    pool_params: OsmosisPoolParams,
    future_pool_governor: String,
    #[serde(alias = "totalShares")]
    total_shares: OsmosisPoolToken,
    #[serde(alias = "poolAssets")]
    pool_assets: Vec<OsmosisPoolAssets>,
    #[serde(alias = "totalWeight")]
    total_weight: String,
}

#[derive(Debug)]
pub struct OsmosisPoolConfig {
    pub estimate_quote: bool,
}

impl OsmosisPool {
    fn calculate_quote(
        &self,
        amount: u128,
        token_in_amount: u128,
        token_out_amount: u128,
        token_in_weight: u128,
        token_out_weight: u128,
        token_in_decimals: u32,
        token_out_decimals: u32,
    ) -> u128 {
        assert!(
            self.pool_assets.len() == 2,
            "Can only calculate quote for pools with 2 assets, estimate quote instead."
        );
        // only on block by block basis, no time weighted average
        (token_out_amount * token_out_weight * u128::from(token_out_decimals) * amount)
            / (token_in_amount * token_in_weight * u128::from(token_in_decimals))
    }

    async fn estimate_quote(
        &self,
        amount: u128,
        token_in_denom: &str,
        token_out_denom: &str,
    ) -> Result<u128> {
        let mut client =
            QueryClient::connect("https://grpc-osmosis-ia.cosmosia.notional.ventures:443").await?;

        let pool_id = u64::from_str_radix(&self.id, 10)?;
        let request = QuerySwapExactAmountInRequest {
            sender: self.address.clone(), // small hack because it uses SwapExactAmountIn just without writing new state so we need a address with enought liquidity, we assume the pool has that
            pool_id: pool_id,
            token_in: format!("{}{}", amount, token_in_denom),
            routes: vec![SwapAmountInRoute {
                pool_id: pool_id,
                token_out_denom: token_out_denom.to_string(),
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
impl Pool<OsmosisPoolConfig> for OsmosisPool {
    async fn get_quote(
        &self,
        amount: u128,
        token_in_denom: &str,
        token_out_denom: &str,
        config: OsmosisPoolConfig,
    ) -> Result<Quote> {
        if config.estimate_quote {
            Ok(Quote {
                token_in: amount,
                token_out: self
                    .estimate_quote(amount, token_in_denom, token_out_denom)
                    .await?,
            })
        } else {
            let token_in_index = self
                .pool_assets
                .iter()
                .position(|x| {
                    x.token.denom == token_in_denom
                        || (match x.token.native_name.clone() {
                            Some(x) => x == token_in_denom,
                            None => false,
                        })
                })
                .unwrap();
            let token_out_index = self
                .pool_assets
                .iter()
                .position(|x| {
                    x.token.denom == token_out_denom
                        || (match x.token.native_name.clone() {
                            Some(x) => x == token_out_denom,
                            None => false,
                        })
                })
                .unwrap();

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
                token_in: amount,
                token_out: self.calculate_quote(
                    amount,
                    token_in_amount,
                    token_out_amount,
                    token_in_weight,
                    token_out_weight,
                    token_in_decimals,
                    token_out_decimals,
                ),
            })
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OsmosisPoolParams {
    #[serde(alias = "swapFee")]
    swap_fee: String,
    #[serde(alias = "exitFee")]
    exit_fee: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OsmosisPoolToken {
    denom: String,
    amount: String,
    native_name: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OsmosisPoolAssets {
    token: OsmosisPoolToken,
    weight: String,
}

#[derive(Debug, serde::Deserialize)]
struct OsmosisPoolsFetchResult {
    pools: Vec<OsmosisPool>,
}

pub async fn fetch_osmosis_pools() -> Result<()> {
    // TODO: currently only ~800 pools, may need to use pagination
    let resp: OsmosisPoolsFetchResult =
        reqwest::get("https://lcd.osmosis.zone/osmosis/gamm/v1beta1/pools?pagination.limit=1000")
            .await?
            .json()
            .await?;

    let pools_raw = resp.pools;
    // TODO: can we not copy here?
    let mut pools: Vec<OsmosisPool> = vec![];

    // #TODO: this loop is parallelizable but that makes no sense at this time because the api server would rate limit us
    for pool in pools_raw {
        // TODO: this should probably be mapable
        let mut assets: Vec<OsmosisPoolAssets> = vec![];
        for asset in pool.pool_assets {
            sleep(Duration::from_millis(500)).await;
            assets.push(OsmosisPoolAssets {
                token: OsmosisPoolToken {
                    denom: asset.token.denom.clone(),
                    amount: asset.token.amount,
                    native_name: if asset.token.denom.starts_with("ibc/") {
                        // TODO: cache results? there is a endpoint to get all traces but that is missing the hash
                        let native_denom =
                            denom_trace("https://lcd.osmosis.zone", &asset.token.denom[4..])
                                .await?;

                        Some(native_denom.base_denom)
                    } else {
                        Some(asset.token.denom)
                    },
                },
                weight: asset.weight,
            })
        }

        pools.push(OsmosisPool {
            address: pool.address,
            id: pool.id,
            pool_params: pool.pool_params,
            future_pool_governor: pool.future_pool_governor,
            total_shares: pool.total_shares,
            pool_assets: assets,
            total_weight: pool.total_weight,
        })
    }

    let text = serde_json::to_string(&pools)?;
    let path = Path::new("osmosis_pools_hackathon.json");
    //let text = serde_json::to_string(&request)?;
    let mut file = File::create(path)?;
    file.write(text.as_bytes())?;

    Ok(())
}

pub fn load_osmo_pools_from_file(path: &Path) -> Result<Vec<OsmosisPool>> {
    let mut file = File::open(path)?;

    let mut text: String = "".to_string();
    file.read_to_string(&mut text)?;
    let pools: Vec<OsmosisPool> = serde_json::from_str(&text)?;
    Ok(pools)
}
