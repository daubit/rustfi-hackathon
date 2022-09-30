use eyre::Result;
use tracy::juno_pool::query_contract;
use tracy::pools::fetch_osmosis_pools;

fn main() -> Result<()> {
    println!("{}", "oi");
    // fetch_osmosis_pools().await?;
    let res = query_contract(&String::from("https://lcd-juno.itastakers.com"), &String::from("juno1vyvdyd70pz3yhnduzfhl098dk5pfpjl8nxmsrm6gmd3f7y5yrxvqw7e892"), &String::from("eyJ0b2tlbnMiOiB7Im93bmVyIjogImp1bm8xbmt5eWpqZzJ0bnpudThlMGZtc2E5ZW10NzY5Z2hzeDdoaG43a3UifSB9"));
    println!("{}", res.unwrap());
    Ok(())
}
