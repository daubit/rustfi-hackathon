use eyre::Result;
use tracy::juno_pool::{
    fetch_juno_pools, get_contracts, get_pool_info, get_price_for, get_token_info, query_contract,
};
use tracy::pools::fetch_osmosis_pools;

fn main() -> Result<()> {
    // fetch_osmosis_pools().await?;
    let api = "https://lcd-juno.itastakers.com";
    let _res = fetch_juno_pools(api);
    // let res = get_pool_info(api, contract_address);
    // let res = get_contracts(api, 16);
    // println!("{:?}", res.unwrap());
    // let price = get_price_for(api, contract_address, 1000, true);
    // println!("{:?}", price.unwrap());
    Ok(())
}
