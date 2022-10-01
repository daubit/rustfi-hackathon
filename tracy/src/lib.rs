use async_trait::async_trait;
use eyre::Result;



pub struct Quote {
    pub token_in: u128,
    pub token_out: u128,
}

#[async_trait]
pub trait Pool<T> {
    async fn get_quote(
        &self,
        amount: u128,
        token_in_denom: &str,
        token_out_denom: &str,
        config: T,
    ) -> Result<Quote>;
}

pub mod juno_pool;
pub mod pools;
pub mod util;
