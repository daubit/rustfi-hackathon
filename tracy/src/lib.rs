use async_trait::async_trait;
use dyn_clone::DynClone;
use eyre::Result;

pub struct Quote {
    pub token_in: u128,
    pub token_out: u128,
}

#[derive(Debug)]
pub struct PoolConfig {
    pub grpc_url: Option<String>,
    pub rest_url: Option<String>,
    pub rpc_url: Option<String>,
    pub estimate_quote: bool,
}

// Send + Static may be unsafe(probably is) but we  use it in DexAgg and use DexAgg behind a mutex
#[async_trait]
pub trait Pool: DynClone + Send + Sync {
    async fn get_quote(
        &self,
        amount: u128,
        token_in_denom: &str,
        token_out_denom: &str,
        config: PoolConfig,
    ) -> Result<Quote>;

    fn token_denoms(&self) -> Vec<String>;
    fn to_json(&self) -> String;
}

dyn_clone::clone_trait_object!(Pool);

pub mod dex;
pub mod juno_pool;
pub mod pools;
pub mod util;
