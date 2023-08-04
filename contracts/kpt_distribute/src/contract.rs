use crate::error::ContractError;
use crate::handler::{add_rule_config, claim, update_config, update_rule_config};
use crate::helper::BASE_RATE_12;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::querier::{query_claimable_info, query_config, query_rule_info};
use crate::state::{
    store_distribute_config, store_rule_config, store_rule_config_state, DistributeConfig,
    RuleConfig, RuleConfigState,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "kryptonite.finance:kpt-distribute";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let gov = msg.gov.unwrap_or_else(|| info.sender.clone());

    // init rule config && state
    let mut rule_total_amount = 0u128;
    for (rule_type, rule_msg) in msg.rule_configs_map {
        rule_total_amount += rule_msg.rule_total_amount.clone();
        let end_linear_release_time =
            rule_msg.start_linear_release_time + rule_msg.unlock_linear_release_time;
        let linear_release_per_second = rule_msg.unlock_linear_release_amount * BASE_RATE_12
            / u128::from(rule_msg.unlock_linear_release_time);
        let rule_config = RuleConfig {
            rule_name: rule_msg.rule_name,
            rule_owner: rule_msg.rule_owner,
            rule_total_amount: rule_msg.rule_total_amount,
            start_release_amount: rule_msg.start_release_amount,
            lock_start_time: rule_msg.lock_start_time,
            lock_end_time: rule_msg.lock_end_time,
            start_linear_release_time: rule_msg.start_linear_release_time,
            end_linear_release_time,
            unlock_linear_release_amount: rule_msg.unlock_linear_release_amount,
            unlock_linear_release_time: rule_msg.unlock_linear_release_time,
            linear_release_per_second,
        };
        store_rule_config(deps.storage, &rule_type, &rule_config)?;

        let rule_config_state = RuleConfigState {
            is_start_release: false,
            claimed_amount: 0u128,
            released_amount: 0u128,
            last_claim_linear_release_time: 0,
        };
        store_rule_config_state(deps.storage, &rule_type, &rule_config_state)?;
    }
    // init distribute config
    let distribute_config = DistributeConfig {
        gov: gov.clone(),
        total_amount: msg.total_amount,
        distribute_token: msg.distribute_token,
        rules_total_amount: rule_total_amount,
    };

    if distribute_config.total_amount < rule_total_amount {
        return Err(StdError::generic_err(
            "total_amount must be greater than rule_total_amount",
        ));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    store_distribute_config(deps.storage, &distribute_config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "instantiate"),
        ("gov", gov.as_str()),
        (
            "total_amount",
            distribute_config.total_amount.to_string().as_str(),
        ),
        (
            "distribute_token",
            distribute_config.distribute_token.to_string().as_str(),
        ),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Claim { rule_type, msg } => claim(deps, env, info, rule_type, msg),
        ExecuteMsg::UpdateConfig {
            gov,
            distribute_token,
        } => update_config(deps, info, gov, distribute_token),
        ExecuteMsg::UpdateRuleConfig { update_rule_msg } => {
            update_rule_config(deps, info, update_rule_msg)
        }
        ExecuteMsg::AddRuleConfig {
            rule_type,
            rule_msg,
        } => add_rule_config(deps, info, rule_type, rule_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryClaimableInfo { rule_type } => {
            to_binary(&query_claimable_info(deps, env, rule_type)?)
        }
        QueryMsg::QueryRuleInfo { rule_type } => to_binary(&query_rule_info(deps, rule_type)?),
        QueryMsg::QueryConfig {} => to_binary(&query_config(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
