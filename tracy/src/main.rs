use eyre::Result;
use tracy::pools::fetch_osmosis_pools;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "oi");
    fetch_osmosis_pools().await?;

    Ok(())
}
