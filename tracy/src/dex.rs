use std::{collections::HashMap, path::Path, sync::Arc};

use crate::{
    pools::{juno_pool::load_juno_pools_from_file, osmosis_pool::load_osmo_pools_from_file_boxed},
    Pool, PoolConfig,
};
use eyre::Result;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct DexAgg {
    pub pools: Arc<Mutex<Vec<Box<dyn Pool>>>>,
    pub config: HashMap<String, PoolConfig>,
}

impl DexAgg {
    pub fn new() -> Result<Self> {
        let mut osmo_pools =
            load_osmo_pools_from_file_boxed(Path::new("./osmosis_pools_hackathon.json"))?;
        let mut juno_pools = load_juno_pools_from_file(Path::new("./juno_pools.json"))?;
        let mut pools: Vec<Box<dyn Pool>> = osmo_pools
            .drain(..)
            .map(|x| Box::<dyn Pool>::from(x))
            .collect();
        pools.append(
            &mut juno_pools
                .drain(..)
                .map(|x| Box::<dyn Pool>::from(x))
                .collect::<Vec<Box<dyn Pool>>>(),
        );
        let mut config = HashMap::new();
        config.insert(
            "osmosis".to_owned(),
            PoolConfig {
                grpc_url: None,
                rest_url: None,
                rpc_url: None,
                estimate_quote: true,
            },
        );
        config.insert(
            "juno".to_owned(),
            PoolConfig {
                grpc_url: None,
                rest_url: Some("https://lcd-juno.itastakers.com".to_owned()),
                rpc_url: None,
                estimate_quote: true,
            },
        );
        Ok(DexAgg {
            pools: Arc::new(Mutex::new(pools)),
            config: config,
        })
    }

    pub async fn with_denom(&self, denom: &String) -> Vec<Box<dyn Pool>> {
        self.pools
            .lock()
            .await
            .clone()
            .into_iter()
            .filter(|x| x.token_denoms().contains(denom))
            .collect()
    }

    // TODO make &str
    pub async fn with_denoms(&self, denoms: Vec<String>) -> Vec<Box<dyn Pool>> {
        self.pools
            .lock()
            .await
            .clone()
            .into_iter()
            .filter(|x| {
                let token_denoms = x.token_denoms();
                denoms.iter().all(|x| token_denoms.contains(x))
            })
            .collect()
    }

    pub async fn with_address(&self, addr: &str) -> Result<Box<dyn Pool>> {
        let pools = self.pools.lock().await;
        let index = pools
            .iter()
            .position(|x| x.address().unwrap() == addr)
            .unwrap();
        Ok(pools[index].clone())
    }

    pub async fn with_chain(&self, chain: &str) -> Vec<Box<dyn Pool>> {
        self.pools
            .lock()
            .await
            .clone()
            .into_iter()
            .filter(|x| x.chain() == chain)
            .collect()
    }
}
