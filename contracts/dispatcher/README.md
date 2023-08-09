# Dispatcher

This contract mainly releases KOL's currency monthly according to the rules.

## GlobalConfig

### Rust

```rust
pub struct GlobalConfig {
    pub gov: Addr,
    pub claim_token: Addr,
    pub start_time: u64,

    pub end_regret_time: u64,
    pub regret_token_receiver: Addr,

    pub total_lock_amount: Uint256,
    pub total_unlock_amount: Uint256,

    pub start_lock_period_time: u64,
    pub duration_per_period: u64,
    pub periods: u64,
}
```

| Key                      | Type      | Description                                                              |
|--------------------------|-----------|--------------------------------------------------------------------------|
| `gov`                    | `Addr`    | The address of the governance contract                                   |
| `claim_token`            | `Addr`    | The address of the token contract to be claimed                          |
| `start_time`             | `u64`     | The start time of the contract                                           |
| `end_regret_time`        | `u64`     | The end time of the regret period                                        |
| `regret_token_receiver`  | `Addr`    | The address of the token contract to be claimed during the regret period |
| `total_lock_amount`      | `Uint256` | The total amount of tokens to be locked                                  |
| `total_unlock_amount`    | `Uint256` | The total amount of tokens to be unlocked                                |
| `start_lock_period_time` | `u64`     | The start time of the lock period                                        |
| `duration_per_period`    | `u64`     | The duration of each lock period                                         |
| `periods`                | `u64`     | The number of lock periods                                               |

## InstantiateMsg {.tabset}

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub gov: Option<Addr>,
    pub claim_token: Addr,
    pub start_time: u64,

    pub end_regret_time: u64,
    pub regret_token_receiver: Addr,

    pub total_lock_amount: Uint256,
    pub total_unlock_amount: Uint256,

    pub start_lock_period_time: u64,
    pub duration_per_period: u64,
    pub periods: u64,
}
```

### JSON

```json
{
  "gov": "sei1...",
  "claim_token": "sei1...",
  "start_time": "1688128677",
  "end_regret_time": "1690720710",
  "regret_token_receiver": "sei1...",
  "total_lock_amount": "80_000_000_000_000",
  "total_unlock_amount": "20_000_000_000_000",
  "start_lock_period_time": "1688828677",
  "duration_per_period": "2592000",
  "periods": "25"
}
```

| Key                      | Type      | Description                                                              |
|--------------------------|-----------|--------------------------------------------------------------------------|
| `gov`                    | `Addr`    | The address of the governance contract                                   |
| `claim_token`            | `Addr`    | The address of the token contract to be claimed                          |
| `start_time`             | `u64`     | The start time of the contract                                           |
| `end_regret_time`        | `u64`     | The end time of the regret period                                        |
| `regret_token_receiver`  | `Addr`    | The address of the token contract to be claimed during the regret period |
| `total_lock_amount`      | `Uint256` | The total amount of tokens to be locked                                  |
| `total_unlock_amount`    | `Uint256` | The total amount of tokens to be unlocked                                |
| `start_lock_period_time` | `u64`     | The start time of the lock period                                        |
| `duration_per_period`    | `u64`     | The duration of each lock period                                         |
| `periods`                | `u64`     | The number of lock periods                                               |

## ExecuteMsg

### UpdateConfig {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateGlobalConfigMsg),
}
```

#### JSON

```json
{
  "update_config": {}
}
```

### UpdateGlobalConfigMsg {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UpdateGlobalConfigMsg {
    pub gov: Option<Addr>,
    pub claim_token: Option<Addr>,
    pub start_time: Option<u64>,
    pub end_regret_time: Option<u64>,
    pub regret_token_receiver: Option<Addr>,

    pub total_lock_amount: Option<Uint256>,
    pub total_unlock_amount: Option<Uint256>,
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "claim_token": "sei1...",
  "start_time": "1688128677",
  "end_regret_time": "1690720710",
  "regret_token_receiver": "sei1...",
  "total_lock_amount": "80_000_000_000_000",
  "total_unlock_amount": "20_000_000_000_000"
}
```

| Key                     | Type       | Description                                                              |
|-------------------------|------------|--------------------------------------------------------------------------|
| `gov`                   | `Addr`*    | The address of the governance contract                                   |
| `claim_token`           | `Addr`*    | The address of the token contract to be claimed                          |
| `start_time`            | `u64`*     | The start time of the contract                                           |
| `end_regret_time`       | `u64`*     | The end time of the regret period                                        |
| `regret_token_receiver` | `Addr`*    | The address of the token contract to be claimed during the regret period |
| `total_lock_amount`     | `Uint256`* | The total amount of tokens to be locked                                  |
| `total_unlock_amount`   | `Uint256`* | The total amount of tokens to be unlocked                                |

* = optional

### AddUser {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    AddUser(Vec<AddUserMsg>),
}
```

#### JSON

```json
{
  "add_user": []
}
```

### AddUserMsg {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct AddUserMsg {
    pub user: Addr,
    pub unlock_amount: Uint256,
    pub lock_amount: Uint256,
    pub replace: bool,
}
```

#### JSON

```json
{
  "user": "sei1...",
  "unlock_amount": "20_000_000_000_000",
  "lock_amount": "20_000_000_000_000",
  "replace": false
}
```

| Key             | Type      | Description                                                                 |
|-----------------|-----------|-----------------------------------------------------------------------------|
| `user`          | `Addr`    | The address of the user to be added                                         |
| `unlock_amount` | `Uint256` | The amount of tokens to be unlocked                                         |
| `lock_amount`   | `Uint256` | The amount of tokens to be locked                                           |
| `replace`       | `bool`    | Whether to replace the existing user with the same address (default: false) |

### UserRegret {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UserRegret {},
}
```

#### JSON

```json
{
  "user_regret": {}
}
```

### UserClaim {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UserClaim {},
}
```

#### JSON

```json
{
  "user_claim": {}
}
```

### RegretClaim {.tabset}

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    RegretClaim {},
}
```

#### JSON

```json
{
  "regret_claim": {}
}
```

## QueryMsg

### QueryGlobalConfig {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GlobalInfosResponse)]
    QueryGlobalConfig {},
}
```

#### JSON

```json
{
  "query_global_config": {}
}
```

### GlobalInfosResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct GlobalInfosResponse {
    pub config: GlobalConfig,
    pub state: GlobalState,
}
```

#### JSON

```json
{
  "config": {},
  "state": {}
}
```

| Key      | Type           | Description       |
|----------|----------------|-------------------|
| `config` | `GlobalConfig` | The global config |
| `state`  | `GlobalState`  | The global state  |

### GlobalConfig {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalConfig {
    pub gov: Addr,
    pub claim_token: Addr,
    pub start_time: u64,

    pub end_regret_time: u64,
    pub regret_token_receiver: Addr,

    pub total_lock_amount: Uint256,
    pub total_unlock_amount: Uint256,

    pub start_lock_period_time: u64,
    pub duration_per_period: u64,
    pub periods: u64,
}
```

#### JSON

```json
{
  "gov": "sei1...",
  "claim_token": "sei1...",
  "start_time": "1688128677",
  "end_regret_time": "1690720710",
  "regret_token_receiver": "sei1...",
  "total_lock_amount": "80_000_000_000_000",
  "total_unlock_amount": "20_000_000_000_000",
  "start_lock_period_time": "1688128677",
  "duration_per_period": "86400",
  "periods": "30"
}
```

| Key                      | Type      | Description                                                              |
|--------------------------|-----------|--------------------------------------------------------------------------|
| `gov`                    | `Addr`    | The address of the governance contract                                   |
| `claim_token`            | `Addr`    | The address of the token contract to be claimed                          |
| `start_time`             | `u64`     | The start time of the contract                                           |
| `end_regret_time`        | `u64`     | The end time of the regret period                                        |
| `regret_token_receiver`  | `Addr`    | The address of the token contract to be claimed during the regret period |
| `total_lock_amount`      | `Uint256` | The total amount of tokens to be locked                                  |
| `total_unlock_amount`    | `Uint256` | The total amount of tokens to be unlocked                                |
| `start_lock_period_time` | `u64`     | The start time of the lock period                                        |
| `duration_per_period`    | `u64`     | The duration of each lock period                                         |
| `periods`                | `u64`     | The number of lock periods                                               |

### GlobalState {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalState {
    pub total_user_unlock_amount: Uint256,
    pub total_user_claimed_unlock_amount: Uint256,
    pub total_user_lock_amount: Uint256,
    pub total_user_claimed_lock_amount: Uint256,
}
```

#### JSON

```json
{
  "total_user_unlock_amount": "20_000_000_000_000",
  "total_user_claimed_unlock_amount": "0",
  "total_user_lock_amount": "20_000_000_000_000",
  "total_user_claimed_lock_amount": "0"
}
```

| Key                                | Type      | Description                                                                   |
|------------------------------------|-----------|-------------------------------------------------------------------------------|
| `total_user_unlock_amount`         | `Uint256` | The total amount of tokens to be unlocked by all users                        |
| `total_user_claimed_unlock_amount` | `Uint256` | The total amount of tokens to be unlocked by all users that have been claimed |
| `total_user_lock_amount`           | `Uint256` | The total amount of tokens to be locked by all users                          |
| `total_user_claimed_lock_amount`   | `Uint256` | The total amount of tokens to be locked by all users that have been claimed   |

### QueryUserInfo {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserInfoResponse)]
    QueryUserInfo { user: Addr },
}
```

#### JSON

```json
{
  "query_user_info": {
    "user": "sei1..."
  }
}
```

### UserInfoResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct UserInfoResponse {
    pub state: UserState,
    pub current_period: u64,
    pub claimable_lock_amount: Uint256,
    pub claimable_unlock_amount: Uint256,
}
```

#### JSON

```json
{
  "state": {},
  "current_period": "0",
  "claimable_lock_amount": "0",
  "claimable_unlock_amount": "0"
}
```

| Key                       | Type        | Description                                                                |
|---------------------------|-------------|----------------------------------------------------------------------------|
| `state`                   | `UserState` | The user state                                                             |
| `current_period`          | `u64`       | The current lock period                                                    |
| `claimable_lock_amount`   | `Uint256`   | The amount of tokens that can be claimed by the user in the current period |
| `claimable_unlock_amount` | `Uint256`   | The amount of tokens that can be claimed by the user in the current period |

### UserState {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserState {
    pub user: Addr,
    pub total_user_unlock_amount: Uint256,
    pub total_user_lock_amount: Uint256,

    pub claimed_unlock_amount: Uint256,
    pub claimed_lock_amount: Uint256,

    pub last_claimed_period: u64,
    pub user_per_lock_amount: Uint256,

    pub is_regret: bool,
    pub regret_time: u64,
}
```

#### JSON

```json
{
  "user": "sei1...",
  "total_user_unlock_amount": "20_000_000_000_000",
  "total_user_lock_amount": "20_000_000_000_000",
  "claimed_unlock_amount": "0",
  "claimed_lock_amount": "0",
  "last_claimed_period": "0",
  "user_per_lock_amount": "0",
  "is_regret": false,
  "regret_time": "0"
}
```

| Key                        | Type      | Description                                                                  |
|----------------------------|-----------|------------------------------------------------------------------------------|
| `user`                     | `Addr`    | The address of the user                                                      |
| `total_user_unlock_amount` | `Uint256` | The total amount of tokens to be unlocked by the user                        |
| `total_user_lock_amount`   | `Uint256` | The total amount of tokens to be locked by the user                          |
| `claimed_unlock_amount`    | `Uint256` | The total amount of tokens to be unlocked by the user that have been claimed |
| `claimed_lock_amount`      | `Uint256` | The total amount of tokens to be locked by the user that have been claimed   |
| `last_claimed_period`      | `u64`     | The last claimed lock period                                                 |
| `user_per_lock_amount`     | `Uint256` | The amount of tokens to be locked by the user per lock period                |
| `is_regret`                | `bool`    | Whether the user is in the regret period                                     |
| `regret_time`              | `u64`     | The end time of the regret period                                            |

### QueryRegretInfo {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(RegretInfoResponse)]
    QueryRegretInfo {},
}
```

#### JSON

```json
{
  "query_regret_info": {}
}
```

### RegretInfoResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct RegretInfoResponse {
    pub info: RegretInfo,
}
```

#### JSON

```json
{
  "info": {}
}
```

| Key    | Type         | Description     |
|--------|--------------|-----------------|
| `info` | `RegretInfo` | The regret info |

### RegretInfo {.tabset}

#### Rust

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegretInfo {
    pub total_unlock_amount: Uint256,
    pub total_claimed_unlock_amount: Uint256,

    pub last_claimed_period: u64,
    pub per_lock_amount: Uint256,

    pub total_lock_amount: Uint256,
    pub total_claimed_lock_amount: Uint256,
}
```

#### JSON

```json
{
  "total_unlock_amount": "20_000_000_000_000",
  "total_claimed_unlock_amount": "0",
  "last_claimed_period": "0",
  "per_lock_amount": "0",
  "total_lock_amount": "20_000_000_000_000",
  "total_claimed_lock_amount": "0"
}
```

| Key                           | Type      | Description                                                                   |
|-------------------------------|-----------|-------------------------------------------------------------------------------|
| `total_unlock_amount`         | `Uint256` | The total amount of tokens to be unlocked by all users                        |
| `total_claimed_unlock_amount` | `Uint256` | The total amount of tokens to be unlocked by all users that have been claimed |
| `last_claimed_period`         | `u64`     | The last claimed lock period                                                  |
| `per_lock_amount`             | `Uint256` | The amount of tokens to be locked by all users per lock period                |
| `total_lock_amount`           | `Uint256` | The total amount of tokens to be locked by all users                          |
| `total_claimed_lock_amount`   | `Uint256` | The total amount of tokens to be locked by all users that have been claimed   |

### QueryUserInfos {.tabset}

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec < UserInfoResponse >)]
    QueryUserInfos {
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
}
```

#### JSON

```json
{
  "query_user_infos": {
    "start_after": "sei1...",
    "limit": 10
  }
}
```

| Key           | Type    | Description                            |
|---------------|---------|----------------------------------------|
| `start_after` | `Addr`* | The address of the user to start after |
| `limit`       | `u32`*  | The maximum number of users to return  |

* = optional



