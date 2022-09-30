#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DenomTrace {
    path: String,
    base_denom: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DenomTraceRaw {
    denom_trace: DenomTrace,
}

use eyre::Result;

pub async fn denom_trace(api_url: &str, hash: &str) -> Result<DenomTrace> {
    let url = format!(
        "{}/ibc/applications/transfer/v1beta1/denom_traces/{}",
        api_url, hash
    );

    let raw_trace: DenomTraceRaw = reqwest::get(url).await?.json().await?;
    Ok(raw_trace.denom_trace)
}
