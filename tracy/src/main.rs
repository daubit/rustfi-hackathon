use std::path::Path;

use eyre::Result;
use tracy::juno_pool::{extract_assets, fetch_juno_pools, JunoPoolConfig};
use tracy::pools::load_osmo_pools_from_file_boxed;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "oi");
    let pools = load_osmo_pools_from_file_boxed(Path::new("./osmosis_pools_hackathon.json"))?;

    Ok(())
}
