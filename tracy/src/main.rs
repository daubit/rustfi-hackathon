use std::path::Path;

use eyre::Result;
use tracy::juno_pool::{extract_assets, fetch_juno_pools, JunoPool, JunoPoolConfig};
use tracy::Pool;
use tracy::PoolConfig;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "oi");
    let pools = load_osmo_pools_from_file_boxed(Path::new("./osmosis_pools_hackathon.json"))?;
    let quote = pools[0];
    let config = JunoPoolConfig {
        path: "juno_pools.json".to_string(),
        api: api.to_string(),
    };
    let token_in = "ujuno";
    let token_out = "uatom";
    let amount = 1000000;
    let quote = pool.get_quote(amount, token_in, token_out, config).await?;
    println!(
        "Price for {} {} -> {} {}",
        token_in, amount, token_out, quote.token_out
    );
    Ok(())
}
