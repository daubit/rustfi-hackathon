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

use eyre::Result;

#[derive(Debug, serde::Deserialize)]
struct OsmosisPoolsFetchResult {
    pools: Vec<OsmosisPool>,
}

pub async fn fetch_osmosis_pools() -> Result<()> {
    let resp: OsmosisPoolsFetchResult =
        reqwest::get("https://lcd.osmosis.zone/osmosis/gamm/v1beta1/pools?pagination.limit=1000")
            .await?
            .json()
            .await?;

    let pools = resp.pools;
    let text = serde_json::to_string(&pools)?;

    let path = Path::new("osmosis_pools.json");
    //let text = serde_json::to_string(&request)?;
    let mut file = File::create(path)?;
    file.write(text.as_bytes())?;

    Ok(())
}
