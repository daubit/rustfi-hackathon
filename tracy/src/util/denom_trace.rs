#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DenomTrace {
    pub path: String,
    pub base_denom: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DenomTraceRaw {
    denom_trace: DenomTrace,
}

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    time::Duration,
};

use eyre::Result;
use tokio::time::sleep;

pub async fn denom_trace(api_url: &str, hash: &str) -> Result<DenomTrace> {
    let url = format!("{}/ibc/apps/transfer/v1/denom_traces/{}", api_url, hash);
    let raw_trace: DenomTraceRaw = reqwest::get(url).await?.json().await?;
    Ok(raw_trace.denom_trace)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DenomTraceCache {
    pub trace: DenomTrace,
    pub ibc: String,
}

fn denom_trace_cache_contains<'a>(
    cache: &'a Vec<DenomTraceCache>,
    ibc: &'a str,
) -> Option<&'a DenomTraceCache> {
    cache.iter().find(|x| x.ibc == ibc)
}

pub async fn resolve_ibc(
    cache: Vec<DenomTraceCache>,
    api: &str,
    denom: String,
    should_sleep: bool,
) -> Result<(Option<String>, Vec<DenomTraceCache>)> {
    let mut cache = cache;
    if denom.starts_with("ibc/") {
        if let Some(x) = denom_trace_cache_contains(&cache, &denom) {
            Ok((Some(x.trace.base_denom.clone()), cache))
        } else {
            if should_sleep {
                sleep(Duration::from_millis(200)).await;
            }
            let native_denom = denom_trace(api, &denom[4..]).await?;

            cache.push(DenomTraceCache {
                trace: native_denom.clone(),
                ibc: denom.clone(),
            });

            Ok((Some(native_denom.base_denom), cache))
        }
    } else {
        Ok((Some(denom), cache))
    }
}

pub fn load_denom_trace_cache_from_file(path: &Path) -> Result<Vec<DenomTraceCache>> {
    let file = File::open(path);
    if let Err(x) = file {
        println!("could not open trace cache file error: {}", x);
        return Ok(vec![]);
    }

    let mut text: String = "".to_string();
    let read_error = file.unwrap().read_to_string(&mut text);
    if let Err(x) = read_error {
        println!("could not read trace cache file error: {}", x);
        return Ok(vec![]);
    }
    let pools = serde_json::from_str(&text);
    if let Err(x) = pools {
        println!("could not parse trace cache file error: {}", x);
        return Ok(vec![]);
    }
    Ok(pools.unwrap())
}

pub fn save_denom_trace_cache_to_file(path: &Path, cache: Vec<DenomTraceCache>) -> Result<()> {
    let text = serde_json::to_string(&cache)?;
    let mut file = File::create(path)?;
    file.write(text.as_bytes())?;

    Ok(())
}
