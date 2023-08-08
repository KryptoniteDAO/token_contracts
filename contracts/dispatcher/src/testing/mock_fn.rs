use crate::contract::instantiate;
use crate::msg::{AddUserMsg, InstantiateMsg};
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{Addr, Env, MessageInfo, OwnedDeps, Response, Uint256};

pub const CREATOR: &str = "creator";
pub const CLAIM_TOKEN: &str = "claim_token";
pub const REGRET_TOKEN_RECEIVER: &str = "regret_token_receiver";

pub fn mock_instantiate_msg(claim_token: Addr) -> InstantiateMsg {
    InstantiateMsg {
        gov: None,
        claim_token,
        start_time: 1688128677,
        end_regret_time: 1690720710,
        regret_token_receiver: Addr::unchecked(REGRET_TOKEN_RECEIVER.to_string()),
        total_lock_amount: Uint256::from(80_000_000_000_000u128),
        total_unlock_amount: Uint256::from(20_000_000_000_000u128),
        start_lock_period_time: 1688828677,
        // 30 days
        duration_per_period: 86400 * 30,
        periods: 25,
    }
}

pub fn mock_instantiate(
    msg: InstantiateMsg,
) -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    Env,
    MessageInfo,
    Response,
) {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(CREATOR, &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    (deps, env, info, res)
}

pub fn mock_add_users_msg() -> Vec<AddUserMsg> {
    let mut users = Vec::new();
    users.push(AddUserMsg {
        user: Addr::unchecked("tom"),
        lock_amount: Uint256::from(4_000_000_000u128),
        unlock_amount: Uint256::from(1_000_000_000u128),
        replace: false,
    });

    users.push(AddUserMsg {
        user: Addr::unchecked("alice"),
        unlock_amount: Uint256::from(1_100_000_000u128),
        lock_amount: Uint256::from(4_400_000_000u128),
        replace: false,
    });
    users.push(AddUserMsg {
        user: Addr::unchecked("regret_2"),
        unlock_amount: Uint256::from(1_200_000_000u128),
        lock_amount: Uint256::from(4_800_000_000u128),
        replace: false,
    });
    users.push(AddUserMsg {
        user: Addr::unchecked("regret_3"),
        unlock_amount: Uint256::from(1_300_000_000u128),
        lock_amount: Uint256::from(5_200_000_000u128),
        replace: false,
    });
    // add 100 users
    for i in 0..1000 {
        users.push(AddUserMsg {
            user: Addr::unchecked(format!("addr{}", i)),
            unlock_amount: Uint256::from(100_000_000u128),
            lock_amount: Uint256::from(100_000_000u128),
            replace: false,
        });
    }
    users
}
