#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OsmosisPool {
    address: String,
    id: String,
    poolParams: OsmosisPoolParams,
    future_pool_governor: String,
    totalShares: OsmosisPoolToken,
    poolAssets: Vec<OsmosisPoolAssets>,
    totalWeight: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OsmosisPoolParams {
    swapFee: String,
    exitFee: String,
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
