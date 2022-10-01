use std::path::Path;

use eyre::Result;
use tracy::pools::{fetch_osmosis_pools, load_osmo_pools_from_file, OsmosisPoolConfig};
use tracy::Pool;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "oi");
    let pools = load_osmo_pools_from_file(Path::new("./osmosis_pools_hackathon.json"))?;
    let quote = pools[0]
        .get_quote(
            1000000,
            "uosmo",
            "uatom",
            OsmosisPoolConfig {
                estimate_quote: false,
            },
        )
        .await?;

    println!("{} {}", quote.token_in, quote.token_out);
    //fetch_osmosis_pools().await?;
    Ok(())
}
