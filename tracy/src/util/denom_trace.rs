#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DenomTrace {
    pub path: String,
    pub base_denom: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DenomTraceRaw {
    denom_trace: DenomTrace,
}

use eyre::Result;

pub async fn denom_trace(api_url: &str, hash: &str) -> Result<DenomTrace> {
    let url = format!("{}/ibc/apps/transfer/v1/denom_traces/{}", api_url, hash);

    println!("{}", reqwest::get(url).await?.text().await?);

    //let raw_trace: DenomTraceRaw = reqwest::get(url).await?.json().await?;
    //Ok(raw_trace.denom_trace)
    Ok(DenomTrace {
        path: "()".to_string(),
        base_denom: "()".to_string(),
    })
}
