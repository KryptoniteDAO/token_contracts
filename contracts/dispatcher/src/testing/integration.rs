use crate::msg::{AddUserMsg, GlobalInfosResponse, RegretInfoResponse, UserInfoResponse};
use crate::testing::mock_fn::{mock_add_users_msg, CREATOR, REGRET_TOKEN_RECEIVER};
use crate::testing::mock_third_fn::mock_cw20_instantiate_msg;
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::{Addr, Coin, Response, StdError, StdResult, Timestamp, Uint128, Uint256};
use cw20::BalanceResponse;
use cw_multi_test::{App, AppBuilder, ContractWrapper, Executor};

fn mock_app(owner: Addr, coins: Vec<Coin>, block_time: Option<u64>) -> App {
    let mut block = mock_env().block;
    if let Some(time) = block_time {
        block.time = Timestamp::from_seconds(time);
    }
    AppBuilder::new()
        .with_block(block)
        .build(|router, _, storage| router.bank.init_balance(storage, &owner, coins).unwrap())
}

#[test]
fn test_integration() {
    let block_time = 1688128676u64;
    let creator = Addr::unchecked(CREATOR);
    let regret_token_receiver = Addr::unchecked(REGRET_TOKEN_RECEIVER.clone());
    let tom_address = Addr::unchecked("tom");
    let alice_address = Addr::unchecked("alice");
    let regret_2_address = Addr::unchecked("regret_2");
    let regret_3_address = Addr::unchecked("regret_3");
    let mut app = mock_app(creator.clone(), vec![], Some(block_time));

    // init cw20 token
    let cw20_contract_id = store_cw20_contract(&mut app);
    let cw20instance_msg: cw20_base::msg::InstantiateMsg = mock_cw20_instantiate_msg();
    let cw20_token = app
        .instantiate_contract(
            cw20_contract_id,
            creator.clone(),
            &cw20instance_msg,
            &[], // no funds
            String::from("cw20_token"),
            None,
        )
        .unwrap();

    // init dispatcher contract
    let dispatcher_contract_id = store_dispatcher_contract(&mut app);
    let dispatcher_instance_msg: crate::msg::InstantiateMsg =
        crate::testing::mock_fn::mock_instantiate_msg(cw20_token.clone());
    let dispatcher_contact = app
        .instantiate_contract(
            dispatcher_contract_id,
            creator.clone(),
            &dispatcher_instance_msg,
            &[], // no funds
            String::from("dispatcher_contact"),
            None,
        )
        .unwrap();

    // transfer cw20 token to dispatcher contract
    let transfer_amount = Uint128::from(10_000_000_000_000u128);
    let res = transfer_token(
        &creator,
        &dispatcher_contact,
        &mut app,
        &cw20_token,
        transfer_amount,
    );
    assert!(res.is_ok());

    // check dispatcher token balance
    let res = get_token_balance(&mut app, &cw20_token, &dispatcher_contact);
    assert_eq!(res.balance, transfer_amount);

    // not add user yet , so can not claim
    let add_user_msg = mock_add_users_msg();
    let res = add_users(&creator, &mut app, &add_user_msg, &dispatcher_contact);
    assert!(res.is_ok());

    // not start time , so can not claim
    let res = user_claim(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_err());

    // not start time , so can not regret
    let res = user_regret(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_err());

    // not start time , so can not regret claim
    let res = regret_claim(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_err());

    // start time
    // update block time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1688128677 + 1000u64);
        block.height += 1000000u64;
    });

    // can not add user , because not enough token
    let res = user_claim(
        &Addr::unchecked("unknown address"),
        &mut app,
        &dispatcher_contact,
    );
    assert!(res.is_err());

    // tom can claim 1_000_000_000u128
    let res = user_claim(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_ok());
    // check tom token balance
    let res = get_token_balance(&mut app, &cw20_token, &tom_address);
    let tom_token_balance = res.balance;
    assert_eq!(tom_token_balance, Uint128::from(1_000_000_000u128));

    // check contract token balance
    let res = get_token_balance(&mut app, &cw20_token, &dispatcher_contact);
    assert_eq!(res.balance, transfer_amount - tom_token_balance);

    // tom can't regret
    let res = user_regret(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_err());

    // tom claim again, claim zero.
    let res = user_claim(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_err());

    // alice regret
    let res = user_regret(&alice_address, &mut app, &dispatcher_contact);
    assert!(res.is_ok());

    // alice can't claim ,because is regret.
    let res = user_claim(&alice_address, &mut app, &dispatcher_contact);
    assert!(res.is_err());

    // check alice token balance
    let res = get_token_balance(&mut app, &cw20_token, &alice_address);
    let alice_token_balance = res.balance;
    assert_eq!(alice_token_balance, Uint128::from(0u128));

    // check regret info
    let regret_info = query_regret_info(&mut app, &dispatcher_contact);
    let alice_user_info = query_user_info(&mut app, &dispatcher_contact, &alice_address);
    assert_eq!(
        regret_info.info.total_unlock_amount,
        alice_user_info.state.total_user_unlock_amount
    );
    assert_eq!(
        regret_info.info.total_lock_amount,
        alice_user_info.state.total_user_lock_amount
    );
    assert_eq!(
        regret_info.info.per_lock_amount,
        alice_user_info.state.user_per_lock_amount
    );

    // regret claim
    let res = regret_claim(&alice_address, &mut app, &dispatcher_contact);
    assert!(res.is_ok());

    // check regret info
    let regret_info = query_regret_info(&mut app, &dispatcher_contact);
    assert_eq!(
        regret_info.info.total_unlock_amount,
        Uint256::from(1100000000u128)
    );
    assert_eq!(
        regret_info.info.total_claimed_unlock_amount,
        Uint256::from(1100000000u128)
    );
    assert_eq!(
        regret_info.info.total_claimed_lock_amount,
        Uint256::from(0u128)
    );

    // check regret_token_receiver balance
    let res = get_token_balance(&mut app, &cw20_token, &regret_token_receiver);
    let regret_token_receiver_balance = res.balance;
    assert_eq!(regret_token_receiver_balance, Uint128::from(1100000000u128));

    // check contract balance
    let res = get_token_balance(&mut app, &cw20_token, &dispatcher_contact);
    assert_eq!(
        res.balance,
        transfer_amount - regret_token_receiver_balance - tom_token_balance
    );

    // regret_2_address regret
    let res = user_regret(&regret_2_address, &mut app, &dispatcher_contact);
    assert!(res.is_ok());

    //check regret info
    let regret_info = query_regret_info(&mut app, &dispatcher_contact);
    let regret_2_user_info = query_user_info(&mut app, &dispatcher_contact, &regret_2_address);
    assert_eq!(
        regret_info.info.total_unlock_amount,
        alice_user_info.state.total_user_unlock_amount
            + regret_2_user_info.state.total_user_unlock_amount
    );
    assert_eq!(
        regret_info.info.total_lock_amount,
        alice_user_info.state.total_user_lock_amount
            + regret_2_user_info.state.total_user_lock_amount
    );
    assert_eq!(
        regret_info.info.per_lock_amount,
        alice_user_info.state.user_per_lock_amount + regret_2_user_info.state.user_per_lock_amount
    );

    // update block time to after regret time
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1690720710 + 1000u64);
        block.height += 1000000u64;
    });

    // regret_3_address regret
    let res = user_regret(&regret_3_address, &mut app, &dispatcher_contact);
    assert!(res.is_err());

    // update block time to after lock time (3 months)
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1688828677 + 100u64 + 86400 * 30 * 3);
        block.height += 1000000u64;
    });
    // query tom info
    let tom_info = query_user_info(&mut app, &dispatcher_contact, &tom_address);

    // tom claim
    let res = user_claim(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_ok());

    let tom_info_after = query_user_info(&mut app, &dispatcher_contact, &tom_address);

    // check tom token balance
    let res = get_token_balance(&mut app, &cw20_token, &tom_address);
    let tom_token_balance = Uint256::from(res.balance);
    assert_eq!(
        tom_info_after.state.claimed_lock_amount,
        Uint256::from(tom_info_after.current_period) * tom_info.state.user_per_lock_amount
    );
    assert_eq!(
        tom_info_after.state.last_claimed_period,
        tom_info_after.current_period
    );
    assert_eq!(
        tom_token_balance,
        tom_info_after.state.claimed_lock_amount + tom_info_after.state.claimed_unlock_amount
    );

    // update block time to after lock time (26 months)
    app.update_block(|block| {
        block.time = Timestamp::from_seconds(1688828677 + 100u64 + 86400 * 30 * 26);
        block.height += 1000000u64;
    });

    // tom claim
    let res = user_claim(&tom_address, &mut app, &dispatcher_contact);
    assert!(res.is_ok());

    // check tom token balance
    let tom_info_after = query_user_info(&mut app, &dispatcher_contact, &tom_address);

    let res = get_token_balance(&mut app, &cw20_token, &tom_address);
    let tom_token_balance = Uint256::from(res.balance);
    assert_eq!(
        tom_info_after.state.claimed_lock_amount,
        Uint256::from(tom_info_after.current_period) * tom_info_after.state.user_per_lock_amount
    );

    assert_eq!(
        tom_info_after.state.last_claimed_period,
        tom_info_after.current_period
    );
    assert_eq!(
        tom_token_balance,
        tom_info_after.state.claimed_lock_amount + tom_info_after.state.claimed_unlock_amount
    );
    assert_eq!(tom_info_after.state.last_claimed_period, 25);

    // regret claim
    let rest = regret_claim(&Addr::unchecked("adsf"), &mut app, &dispatcher_contact);
    assert!(rest.is_ok());

    // regret info
    let regret_info = query_regret_info(&mut app, &dispatcher_contact);

    // check regret_token_receiver balance
    let res = get_token_balance(&mut app, &cw20_token, &regret_token_receiver);
    let regret_token_receiver_balance = Uint256::from(res.balance);
    assert_eq!(
        regret_info.info.total_claimed_lock_amount + regret_info.info.total_claimed_unlock_amount,
        regret_token_receiver_balance
    );

    // query global infos
    let res = query_global_infos(&mut app, &dispatcher_contact);
    println!("global infos: {:?}", res);

    // query all users
    let users = query_user_infos(&mut app, &dispatcher_contact, None, Some(3));
    assert_eq!(users.len(), 3);

    let users = query_user_infos(
        &mut app,
        &dispatcher_contact,
        Some(Addr::unchecked("addr5")),
        Some(5),
    );
    assert_eq!(users.len(), 5);
}

fn add_users(
    user: &Addr,
    app: &mut App,
    add_user_msgs: &Vec<AddUserMsg>,
    dispatcher_contract: &Addr,
) -> StdResult<Response> {
    let send_msg = crate::msg::ExecuteMsg::AddUser(add_user_msgs.clone());
    let res = app.execute_contract(
        user.clone(),
        dispatcher_contract.clone(),
        &send_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("add_users error: {:?}", res);
        Err(StdError::generic_err("add_users error"))
    } else {
        Ok(Response::default())
    }
}
fn user_regret(user: &Addr, app: &mut App, dispatcher_contract: &Addr) -> StdResult<Response> {
    let send_msg = crate::msg::ExecuteMsg::UserRegret {};
    let res = app.execute_contract(
        user.clone(),
        dispatcher_contract.clone(),
        &send_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("user_regret error: {:?}", res);
        Err(StdError::generic_err("user_regret error"))
    } else {
        Ok(Response::default())
    }
}
fn user_claim(user: &Addr, app: &mut App, dispatcher_contract: &Addr) -> StdResult<Response> {
    let send_msg = crate::msg::ExecuteMsg::UserClaim {};
    let res = app.execute_contract(
        user.clone(),
        dispatcher_contract.clone(),
        &send_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("user_claim error: {:?}", res);
        Err(StdError::generic_err("user_claim error"))
    } else {
        Ok(Response::default())
    }
}
fn regret_claim(user: &Addr, app: &mut App, dispatcher_contract: &Addr) -> StdResult<Response> {
    let send_msg = crate::msg::ExecuteMsg::RegretClaim {};
    let res = app.execute_contract(
        user.clone(),
        dispatcher_contract.clone(),
        &send_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("regret_claim error: {:?}", res);
        Err(StdError::generic_err("regret_claim error"))
    } else {
        Ok(Response::default())
    }
}

fn query_global_infos(app: &mut App, dispatcher_contract: &Addr) -> GlobalInfosResponse {
    let query_msg = crate::msg::QueryMsg::QueryGlobalConfig {};
    let res: GlobalInfosResponse = app
        .wrap()
        .query_wasm_smart(dispatcher_contract.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn query_user_info(app: &mut App, dispatcher_contract: &Addr, user: &Addr) -> UserInfoResponse {
    let query_msg = crate::msg::QueryMsg::QueryUserInfo { user: user.clone() };
    let res: UserInfoResponse = app
        .wrap()
        .query_wasm_smart(dispatcher_contract.clone().to_string(), &query_msg)
        .unwrap();
    res
}
fn query_regret_info(app: &mut App, dispatcher_contract: &Addr) -> RegretInfoResponse {
    let query_msg = crate::msg::QueryMsg::QueryRegretInfo {};
    let res: RegretInfoResponse = app
        .wrap()
        .query_wasm_smart(dispatcher_contract.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn query_user_infos(
    app: &mut App,
    dispatcher_contract: &Addr,
    start_after: Option<Addr>,
    limit: Option<u32>,
) -> Vec<UserInfoResponse> {
    let query_msg = crate::msg::QueryMsg::QueryUserInfos { start_after, limit };
    let res: Vec<UserInfoResponse> = app
        .wrap()
        .query_wasm_smart(dispatcher_contract.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn store_dispatcher_contract(app: &mut App) -> u64 {
    let contract = Box::new(ContractWrapper::new_with_empty(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    ));
    app.store_code(contract)
}

fn store_cw20_contract(app: &mut App) -> u64 {
    let contract = Box::new(ContractWrapper::new_with_empty(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    ));
    app.store_code(contract)
}

fn get_token_balance(app: &mut App, token: &Addr, user: &Addr) -> BalanceResponse {
    let query_msg = cw20_base::msg::QueryMsg::Balance {
        address: user.clone().to_string(),
    };

    let res: BalanceResponse = app
        .wrap()
        .query_wasm_smart(token.clone().to_string(), &query_msg)
        .unwrap();
    res
}

fn transfer_token(
    from: &Addr,
    to: &Addr,
    app: &mut App,
    cw20_token: &Addr,
    amount: Uint128,
) -> StdResult<Response> {
    let send_msg = cw20_base::msg::ExecuteMsg::Transfer {
        recipient: to.to_string(),
        amount,
    };
    let res = app.execute_contract(
        from.clone(),
        cw20_token.clone(),
        &send_msg,
        &[], // no funds
    );
    if res.is_err() {
        println!("transfer_token error: {:?}", res);
        Err(StdError::generic_err("transfer_token error"))
    } else {
        Ok(Response::default())
    }
}
