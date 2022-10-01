use eyre::Result;
use tracy::juno_pool::{get_token_info, query_contract, get_pool_info, get_contracts};
use tracy::pools::fetch_osmosis_pools;

fn main() -> Result<()> {
    // fetch_osmosis_pools().await?;
    let contract_address = "juno14mdhwtxfywk6tyyexqx5ju5qqtzzh8gj0g0c9rmj04ms8pc7xjkqxnsg2a";
    let api = "https://lcd-juno.itastakers.com";
    let res = get_pool_info(api, contract_address);
    // let res = get_contracts(api, 16);
    println!("{:?}", res.unwrap());
    Ok(())
}
