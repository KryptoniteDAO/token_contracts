use cw20_base::ContractError;
use crate::helper::is_empty_str;
use crate::mint_receiver::Cw20MintReceiveMsg;
use crate::state::{read_kpt_config, store_kpt_config};
use cosmwasm_std::{attr, Addr, Binary, DepsMut, Env, MessageInfo, Response, StdError, Uint128};
use cw20_base::contract::{execute_burn, execute_mint};

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    kpt_fund: Option<Addr>,
    gov: Option<Addr>,
    kpt_distribute: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut kpt_config = read_kpt_config(deps.storage)?;

    if info.sender != kpt_config.gov {
        return Err(ContractError::Unauthorized {});
    }

    let mut attrs = vec![
        attr("action", "update_config"),
        attr("sender", info.sender.to_string()),
    ];

    if let Some(kpt_fund) = kpt_fund {
        kpt_config.kpt_fund = kpt_fund.clone();
        attrs.push(attr("kpt_fund", kpt_fund.to_string()));
    }
    if let Some(gov) = gov {
        kpt_config.gov = gov.clone();
        attrs.push(attr("gov", gov.to_string()));
    }
    if let Some(kpt_distribute) = kpt_distribute {
        kpt_config.kpt_distribute = kpt_distribute.clone();
        attrs.push(attr("kpt_distribute", kpt_distribute.to_string()));
    }

    store_kpt_config(deps.storage, &kpt_config)?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: Addr,
    amount: Uint128,
    contract: Option<String>,
    msg: Option<Binary>,
) -> Result<Response, ContractError> {
    let msg_sender = info.sender;
    let kpt_config = read_kpt_config(deps.storage)?;
    let kpt_fund = kpt_config.kpt_fund;
    let kpt_distribute = kpt_config.kpt_distribute;

    if is_empty_str(kpt_fund.as_str()) && is_empty_str(kpt_distribute.as_str()) {
        return Err(ContractError::Std(StdError::generic_err("mint contract not configured")));
    }

    if msg_sender.ne(&kpt_fund.clone()) && msg_sender.ne(&kpt_distribute) {
        return Err(ContractError::Unauthorized {});
    }

    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };

    let cw20_res = execute_mint(
        deps.branch(),
        env,
        sub_info,
        user.clone().to_string(),
        amount.clone(),
    );
    if cw20_res.is_err() {
        return Err(ContractError::Std(StdError::generic_err(
            cw20_res.err().unwrap().to_string(),
        )));
    }

    let mut res = cw20_res.unwrap();

    if let Some(contract) = contract {
        if let Some(msg) = msg {
            res = res.add_message(
                Cw20MintReceiveMsg {
                    sender: msg_sender.into(),
                    amount,
                    msg,
                }
                .into_cosmos_msg(contract)?,
            );
        }
    }

    Ok(res)
}

pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: Addr,
    amount: u128,
) -> Result<Response, ContractError> {
    let kpt_config = read_kpt_config(deps.storage)?;
    let msg_sender = info.sender;
    let kpt_fund = kpt_config.kpt_fund;

    if msg_sender != kpt_fund.clone() {
        return Err(ContractError::Unauthorized {});
    }

    let sub_info = MessageInfo {
        sender: user,
        funds: vec![],
    };
    execute_burn(deps, env.clone(), sub_info, Uint128::from(amount))
}
