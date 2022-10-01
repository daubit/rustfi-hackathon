use std::path::Path;

use crate::{juno_pool::load_juno_pools_from_file, pools::load_osmo_pools_from_file_boxed, Pool};
use eyre::Result;

#[derive(Clone)]
pub struct DexAgg {
    pub pools: Vec<Box<dyn Pool>>,
}

impl DexAgg {
    pub fn new() -> Result<Self> {
        let mut osmo_pools =
            load_osmo_pools_from_file_boxed(Path::new("./osmosis_pools_hackathon.json"))?;
        let mut juno_pools = load_juno_pools_from_file(Path::new("./juno_pools_hackathon.json"))?;
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
        Ok(DexAgg { pools: pools })
    }

    pub fn with_denom(&self, denom: &String) -> Vec<Box<dyn Pool>> {
        self.pools
            .clone()
            .into_iter()
            .filter(|x| x.token_denoms().contains(denom))
            .collect()
    }

    pub fn with_denoms(&self, denoms: Vec<String>) -> Vec<Box<dyn Pool>> {
        self.pools
            .clone()
            .into_iter()
            .filter(|x| {
                let token_denoms = x.token_denoms();
                denoms.iter().all(|x| token_denoms.contains(x))
            })
            .collect()
    }
}
