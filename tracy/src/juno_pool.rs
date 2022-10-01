use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::{self, from_utf8};

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmContractResponse {
    pub result: WasmContractRaw,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmContractRaw {
    pub smart: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JunoToken {
    name: Option<String>,
    symbol: Option<String>,
    total_supply: Option<String>,
    address: Option<String>,
    decimals: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JunoPool {
    lp_token_address: String,
    lp_token_supply: String,
    token1_denom: JunoDenom,
    token1_reserve: String,
    token2_denom: JunoDenom,
    token2_reserve: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JunoDenom {
    native: Option<String>,
    cw20: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmCodeContracts {
    result: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmPoolPriceResponse {
    token1_amount: Option<String>,
    token2_amount: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmErrorResponse {
    error: String,
}

#[tokio::main]
async fn get_query(url: &str, query: &Vec<(&str, &str)>) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url).query(query).send().await?.text().await?;
    Ok(resp)
}

pub fn get_contracts(api: &str, code_id: u64) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/wasm/code/{}/contracts", &api.to_string(), code_id);
    let res = get_query(&url, &vec![])?;
    let res = serde_json::from_str::<WasmCodeContracts>(res.as_str())?;
    Ok(res.result)
}

pub fn query_contract(
    api: &str,
    contract_address: &str,
    msg: &str,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/wasm/contract/{}/smart/{}", api, contract_address, msg);
    let res = get_query(&url, &vec![("encoding", "base64")])?;
    let err = res.clone();
    if let Ok(res) = serde_json::from_str::<WasmErrorResponse>(&err) {
        return Err(Box::from(res.error));
    }
    let res = serde_json::from_str::<WasmContractResponse>(&res)?;
    Ok(res.result.smart)
}

pub fn get_token_info(api: &str, contract_address: &str) -> Result<JunoToken, Box<dyn Error>> {
    let msg = "{ \"token_info\" : {} }";
    let msg = base64::encode(msg);
    let res = query_contract(api, contract_address, msg.as_str())?;
    let decoded = base64::decode_config(res, base64::STANDARD)?;
    let decoded = from_utf8(&decoded)?;
    let mut token = serde_json::from_str::<JunoToken>(&decoded)?;
    token.address = Some(contract_address.to_string());
    Ok(token)
}

pub fn get_pool_info(api: &str, contract_address: &str) -> Result<JunoPool, Box<dyn Error>> {
    let msg = "{ \"info\" : {} }";
    let msg = base64::encode(msg);
    let res = query_contract(api, contract_address, msg.as_str())?;
    let decoded = base64::decode_config(res, base64::STANDARD)?;
    let decoded = from_utf8(&decoded)?;
    let pool = serde_json::from_str::<JunoPool>(&decoded)?;
    Ok(pool)
}

pub fn get_price_for(
    api: &str,
    contract_address: &str,
    amount: u64,
    for2: bool,
) -> Result<String, Box<dyn Error>> {
    let (method, arg): (&str, &str) = if for2 {
        ("token2_for_token1_price", "token2_amount")
    } else {
        ("token1_for_token2_price", "token1_amount")
    };
    let msg = format!("{{ \"{}\" : {{ \"{}\": \"{}\" }} }}", method, arg, amount);
    let msg = base64::encode(msg);
    let res = query_contract(api, contract_address, msg.as_str())?;
    let decoded = base64::decode_config(res, base64::STANDARD)?;
    let decoded = from_utf8(&decoded)?;
    let res = serde_json::from_str::<WasmPoolPriceResponse>(&decoded)?;
    if let Some(amount) = res.token1_amount {
        return Ok(amount);
    }
    if let Some(amount) = res.token2_amount {
        return Ok(amount);
    }
    return Err(Box::from("We should not be here"));
}

pub fn fetch_juno_pools(api: &str) -> Result<Vec<JunoPool>, Box<dyn Error>> {
    let contracts = get_contracts(api, 16)?;
    let mut res = Vec::new();
    for contract in contracts {
        let pool = get_pool_info(api, contract.as_str())?;
        res.push(pool);
    }
    let text = serde_json::to_string(&res)?;
    let path = Path::new("juno_pools.json");
    //let text = serde_json::to_string(&request)?;
    let mut file = File::create(path)?;
    file.write(text.as_bytes())?;
    Ok(res)
}
