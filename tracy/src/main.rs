use std::path::Path;

use eyre::Result;
use tracy::pools::load_osmo_pools_from_file_boxed;
use tracy::Pool;
use tracy::PoolConfig;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "oi");
    let pools = load_osmo_pools_from_file_boxed(Path::new("./osmosis_pools_hackathon.json"))?;
    let quote = pools[0]
        .get_quote(
            1000000,
            "uosmo",
            "uatom",
            PoolConfig {
                estimate_quote: false,
                grpc_url: None,
                rest_url: None,
                rpc_url: None,
            },
        )
        .await?;

    println!("{} {}", quote.token_in, quote.token_out);
    //fetch_osmosis_pools().await?;
    Ok(())
}
