use eyre::Result;
use tracy::pools::osmosis_pool::fetch_osmosis_pools;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "oi");
    fetch_osmosis_pools("https://lcd.osmosis.zone").await?;
    Ok(())
}
