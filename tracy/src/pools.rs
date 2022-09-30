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

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use eyre::Result;
use tokio::time::sleep;

use crate::util::denom_trace::denom_trace;

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
    let path = Path::new("osmosis_pools.json");
    //let text = serde_json::to_string(&request)?;
    let mut file = File::create(path)?;
    file.write(text.as_bytes())?;

    Ok(())
}
