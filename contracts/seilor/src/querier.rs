use crate::msg::SeilorConfigResponse;
use crate::state::{read_seilor_config, SeilorConfig};
use cosmwasm_std::{Deps, StdResult};

pub fn query_seilor_config(deps: Deps) -> StdResult<SeilorConfigResponse> {
    let config: SeilorConfig = read_seilor_config(deps.storage)?;
    Ok(SeilorConfigResponse {
        max_supply: config.max_supply,
        seilor_fund: config.seilor_fund,
        seilor_distribute: config.seilor_distribute,
        gov: config.gov,
    })
}
