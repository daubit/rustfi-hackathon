use serde::__private::from_utf8_lossy;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::str;

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
    symbol: String,
    amount: String,
    address: String,
    decimal: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JunoPool {
    lp_address: String,
    token1: JunoToken,
    token2: JunoToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmCodeContracts {
    result: Vec<String>,
}

#[tokio::main]
async fn get_query(url: &String, query: &Vec<(&str, &str)>) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url).query(query).send().await?.text().await?;
    Ok(resp)
}

pub fn get_contracts(api: &str, code_id: u64) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/wasm/code/{}", &api.to_string(), code_id);
    let res = get_query(&url, &vec![])?;
    let res = serde_json::from_str::<WasmCodeContracts>(res.as_str())?;
    Ok(res.result)
}

pub fn query_contract(
    api: &String,
    contract_address: &str,
    msg: &str,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/wasm/contract/{}/smart/{}", api, contract_address, msg);
    let res = get_query(&url, &vec![("encoding", "base64")])?;
    let res = serde_json::from_str::<WasmContractResponse>(&res)?;
    Ok(res.result.smart)
}

// pub fn fetch_juno_pools(api: &str) -> Vec<JunoPool> {
//     let resp: WasmContractResponse =
//         reqwest::get("https://lcd.osmosis.zone/osmosis/gamm/v1beta1/pools?pagination.limit=1000")
//             .await?
//             .json()
//             .await?;
// }
