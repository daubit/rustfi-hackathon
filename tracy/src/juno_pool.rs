use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use eyre::{eyre, Result};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::{self, from_utf8};

use crate::util::denom_trace::denom_trace;
use crate::{Pool, PoolConfig, Quote};

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmContractResponse {
    pub result: WasmContractRaw,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmContractRaw {
    pub smart: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct JunoToken {
    name: Option<String>,
    symbol: Option<String>,
    total_supply: Option<String>,
    address: Option<String>,
    decimals: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WasmPool {
    pub pool_address: Option<String>,
    pub lp_token_address: String,
    pub lp_token_supply: String,
    pub token1: Option<JunoToken>,
    pub token1_denom: JunoDenom,
    pub token1_reserve: String,
    pub token2: Option<JunoToken>,
    pub token2_denom: JunoDenom,
    pub token2_reserve: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

async fn get_query(url: &str, query: &Vec<(&str, &str)>) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url).query(query).send().await?.text().await?;
    Ok(resp)
}

pub async fn get_contracts(api: &str, code_id: u64) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/wasm/code/{}/contracts", &api.to_string(), code_id);
    let res = get_query(&url, &vec![]).await?;
    let res = serde_json::from_str::<WasmCodeContracts>(res.as_str())?;
    Ok(res.result)
}

pub async fn query_contract(
    api: &str,
    contract_address: &str,
    msg: &str,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/wasm/contract/{}/smart/{}", api, contract_address, msg);
    let res = get_query(&url, &vec![("encoding", "base64")]).await?;
    let err = res.clone();
    if let Ok(res) = serde_json::from_str::<WasmErrorResponse>(&err) {
        return Err(Box::from(res.error));
    }
    let res = serde_json::from_str::<WasmContractResponse>(&res)?;
    Ok(res.result.smart)
}

pub async fn get_token_info(
    api: &str,
    contract_address: &str,
) -> Result<JunoToken, Box<dyn Error>> {
    let msg = "{ \"token_info\" : {} }";
    let msg = base64::encode(msg);
    let res = query_contract(api, contract_address, msg.as_str()).await?;
    let decoded = base64::decode_config(res, base64::STANDARD)?;
    let decoded = from_utf8(&decoded)?;
    let mut token = serde_json::from_str::<JunoToken>(&decoded)?;
    token.address = Some(contract_address.to_string());
    Ok(token)
}

pub async fn get_pool_info(api: &str, contract_address: &str) -> Result<WasmPool, Box<dyn Error>> {
    let msg = "{ \"info\" : {} }";
    let msg = base64::encode(msg);
    let res = query_contract(api, contract_address, msg.as_str()).await?;
    let decoded = base64::decode_config(res, base64::STANDARD)?;
    let decoded = from_utf8(&decoded)?;
    let pool = serde_json::from_str::<WasmPool>(&decoded)?;
    Ok(pool)
}

pub async fn get_price_for(
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
    let res = query_contract(api, contract_address, msg.as_str()).await?;
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

pub async fn fetch_juno_pools(api: &str) -> Result<Vec<WasmPool>, Box<dyn Error>> {
    let contracts = get_contracts(api, 16).await?;
    let mut res = Vec::new();
    for contract in contracts {
        let mut pool = get_pool_info(api, contract.as_str()).await?;
        if pool.token1_reserve == "0" || pool.token2_reserve == "0" {
            continue; // Empty pool, probably invalid
        }
        pool.pool_address = Some(contract);
        pool.token1 = Some(extract_token(api, &pool.token1_denom).await?);
        pool.token2 = Some(extract_token(api, &pool.token2_denom).await?);
        res.push(pool);
    }
    let out = serde_json::to_string(&res)?;
    let path = Path::new("juno_pools.json");
    let mut file = File::create(path)?;
    file.write(out.as_bytes())?;
    Ok(res)
}

async fn extract_token(api: &str, denom: &JunoDenom) -> Result<JunoToken, Box<dyn Error>> {
    if let Some(address) = &denom.cw20 {
        return Ok(get_token_info(api, &address).await?);
    }
    if let Some(address) = &denom.native {
        if address == "ujuno" {
            return Ok(JunoToken {
                symbol: Some("ujuno".to_owned()),
                name: Some("ujuno".to_owned()),
                total_supply: None,
                address: None,
                decimals: Some(6),
            });
        } else if address.starts_with("ibc") {
            let origin = address.clone();
            let hash = address.split("/").collect::<Vec<&str>>()[1];
            let denom = denom_trace(api, hash).await?;
            return Ok(JunoToken {
                symbol: Some(denom.base_denom),
                name: None,
                total_supply: None,
                address: Some(origin),
                decimals: Some(6),
            });
        } else if address.starts_with("juno") {
            return Ok(get_token_info(api, &address).await?);
        }
    }
    return Err(Box::from("We should not be here"));
}

pub async fn extract_assets(api: &str) -> Result<(), Box<dyn Error>> {
    let pools = fs::read_to_string(Path::new("juno_pools.json"))?;
    let pools = serde_json::from_str::<Vec<WasmPool>>(&pools)?;
    let mut assets = Vec::new();
    for pool in pools {
        let token1 = extract_token(api, &pool.token1_denom).await?;
        let token2 = extract_token(api, &pool.token2_denom).await?;
        if !assets.contains(&token1) {
            assets.push(token1);
        }
        if !assets.contains(&token2) {
            assets.push(token2);
        }
    }
    let out = serde_json::to_string(&assets)?;
    let path = Path::new("juno_assets.json");
    let mut file = File::create(path)?;
    file.write(out.as_bytes())?;
    Ok(())
}

pub fn load_juno_pools_from_file(path: &Path) -> Result<Vec<Box<WasmPool>>> {
    let mut file = File::open(path)?;
    let mut text: String = "".to_string();
    file.read_to_string(&mut text)?;
    let pools: Vec<Box<WasmPool>> = serde_json::from_str(&text)?;
    Ok(pools)
}

pub fn load_juno_assets_from_file(path: &Path) -> Result<Vec<JunoToken>> {
    let mut file = File::open(path)?;
    let mut text: String = "".to_string();
    file.read_to_string(&mut text)?;
    let pools: Vec<JunoToken> = serde_json::from_str(&text)?;
    Ok(pools)
}

#[async_trait]
impl Pool for WasmPool {
    async fn get_quote(
        &self,
        amount: u128,
        token_in_denom: &str,
        token_out_denom: &str,
        config: PoolConfig,
    ) -> Result<Quote> {
        let token1_denom = &self.token1.clone().unwrap().symbol.unwrap();
        let token2_denom = &self.token2.clone().unwrap().symbol.unwrap();
        let pool_address = self.pool_address.clone().unwrap();
        if (token_in_denom == token1_denom && token_out_denom == token2_denom)
            || (token_in_denom == token1_denom && token_out_denom == token2_denom)
        {
            let amount_out = get_price_for(
                config.rest_url.unwrap().as_str(),
                &pool_address,
                amount as u64,
                token_in_denom == token1_denom,
            )
            .await
            .unwrap();
            let amount_out = amount_out.parse::<u128>()?;
            return Ok(Quote {
                token_in: amount,
                token_out: amount_out,
            });
        }
        Err(eyre!(
            "Cannot find pair: {} | {}",
            token_in_denom,
            token_out_denom
        ))
    }

    fn token_denoms(&self) -> Vec<String> {
        let token1_denom: String = self.token1.clone().unwrap().symbol.unwrap();
        let token2_denom: String = self.token2.clone().unwrap().symbol.unwrap();
        vec![token1_denom, token2_denom]
    }
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn chain(&self) -> String {
        String::from("juno")
    }
}
