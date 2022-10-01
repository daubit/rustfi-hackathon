use eyre::Result;
use tracy::juno_pool::extract_assets;
use tracy::pools::fetch_osmosis_pools;

#[tokio::main]
async fn main() -> Result<()> {
    // fetch_osmosis_pools().await?;
    let api = "https://lcd-juno.itastakers.com";
    let _res = extract_assets(api).await.unwrap();
    // let _res = fetch_juno_pools(api).await.unwrap();
    Ok(())
}
