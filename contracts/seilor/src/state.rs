use cosmwasm_std::{Addr, StdError, StdResult, Storage};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SeilorConfig {
    pub max_supply: u128,
    pub seilor_fund: Addr,
    pub gov: Addr,
    pub seilor_distribute: Addr,
}

const SEILOR_CONFIG: Item<SeilorConfig> = Item::new("seilor_config");

pub fn store_seilor_config(storage: &mut dyn Storage, seilor_config: &SeilorConfig) -> StdResult<()> {
    SEILOR_CONFIG.save(storage, seilor_config)
}

pub fn read_seilor_config(storage: &dyn Storage) -> StdResult<SeilorConfig> {
    SEILOR_CONFIG
        .load(storage)
        .map_err(|_| StdError::generic_err("SeilorConfig not found"))
}
