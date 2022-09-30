use eyre::Result;



pub struct Quote {
    token_in: u128,
    token_out: u128,
}

pub trait Pool {
    fn get_quote(&self) -> Result<Quote>;
}

pub mod juno_pool;
pub mod pools;
pub mod util;
