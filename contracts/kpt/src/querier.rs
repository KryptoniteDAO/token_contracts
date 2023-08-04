use crate::msg::KptConfigResponse;
use crate::state::{read_kpt_config, KptConfig};
use cosmwasm_std::{Deps, StdResult};

pub fn query_kpt_config(deps: Deps) -> StdResult<KptConfigResponse> {
    let config: KptConfig = read_kpt_config(deps.storage)?;
    Ok(KptConfigResponse {
        max_supply: config.max_supply,
        kpt_fund: config.kpt_fund,
        kpt_distribute: config.kpt_distribute,
        gov: config.gov,
    })
}
