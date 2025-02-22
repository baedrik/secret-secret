use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, HumanAddr, StdError, StdResult, Uint128};

use crate::state::{Swap, Tx};
use crate::viewing_key::ViewingKey;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct InitialBalance {
    pub address: HumanAddr,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub admin: HumanAddr,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<InitialBalance>,
    pub prng_seed: String,
    pub config: InitConfig,
}

/// This type represents optional configuration values which can be overridden.
/// All values are optional and have defaults which are more private by default,
/// but can be overridden if necessary
#[derive(Serialize, Deserialize, JsonSchema, Clone, Default)]
pub struct InitConfig {
    /// Indicates whether the total supply is public or should be kept secret.
    /// default: False
    public_total_supply: Option<bool>,
}

impl InitConfig {
    pub fn public_total_supply(&self) -> bool {
        self.public_total_supply.unwrap_or(false)
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    // Native coin interactions
    Redeem {
        amount: Uint128,
        padding: Option<String>,
    },
    Deposit {
        padding: Option<String>,
    },
    Balance {
        padding: Option<String>,
    },

    // Base ERC-20 stuff
    Transfer {
        recipient: HumanAddr,
        amount: Uint128,
        padding: Option<String>,
    },
    Send {
        recipient: HumanAddr,
        amount: Uint128,
        msg: Option<Binary>,
        padding: Option<String>,
    },
    Burn {
        amount: Uint128,
        padding: Option<String>,
    },
    RegisterReceive {
        code_hash: String,
        padding: Option<String>,
    },
    CreateViewingKey {
        entropy: String,
        padding: Option<String>,
    },
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },

    // Allowance
    IncreaseAllowance {
        spender: HumanAddr,
        amount: Uint128,
        expiration: Option<u64>,
        padding: Option<String>,
    },
    DecreaseAllowance {
        spender: HumanAddr,
        amount: Uint128,
        expiration: Option<u64>,
        padding: Option<String>,
    },
    TransferFrom {
        owner: HumanAddr,
        recipient: HumanAddr,
        amount: Uint128,
        padding: Option<String>,
    },
    SendFrom {
        owner: HumanAddr,
        recipient: HumanAddr,
        amount: Uint128,
        msg: Option<Binary>,
        padding: Option<String>,
    },
    BurnFrom {
        owner: HumanAddr,
        amount: Uint128,
        padding: Option<String>,
    },

    // Mint
    Mint {
        amount: Uint128,
        address: HumanAddr,
        padding: Option<String>,
    },

    // Swap
    Swap {
        amount: Uint128,
        network: String,
        destination: String,
        padding: Option<String>,
    },

    // Admin
    ChangeAdmin {
        address: HumanAddr,
        padding: Option<String>,
    },
    SetContractStatus {
        level: ContractStatusLevel,
        padding: Option<String>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    // Native
    Deposit {
        status: ResponseStatus,
    },
    Redeem {
        status: ResponseStatus,
    },
    Balance {
        amount: Uint128,
    },

    // Base
    Transfer {
        status: ResponseStatus,
    },
    Send {
        status: ResponseStatus,
    },
    Burn {
        status: ResponseStatus,
    },
    RegisterReceive {
        status: ResponseStatus,
    },
    CreateViewingKey {
        key: ViewingKey,
    },
    SetViewingKey {
        status: ResponseStatus,
    },

    // Allowance
    IncreaseAllowance {
        spender: HumanAddr,
        owner: HumanAddr,
        allowance: Uint128,
    },
    DecreaseAllowance {
        spender: HumanAddr,
        owner: HumanAddr,
        allowance: Uint128,
    },
    TransferFrom {
        status: ResponseStatus,
    },
    SendFrom {
        status: ResponseStatus,
    },
    BurnFrom {
        status: ResponseStatus,
    },

    // Mint
    Mint {
        status: ResponseStatus,
    },

    // Other
    Swap {
        nonce: u32,
        status: ResponseStatus,
    },
    ChangeAdmin {
        status: ResponseStatus,
    },
    SetContractStatus {
        status: ResponseStatus,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenInfo {},
    ExchangeRate {},
    Swap {
        nonce: u32,
    },
    Allowance {
        owner: HumanAddr,
        spender: HumanAddr,
        padding: Option<String>,
    },
    Balance {
        address: HumanAddr,
        key: String,
    },
    TransferHistory {
        address: HumanAddr,
        key: String,
        page: Option<u32>,
        page_size: u32,
    },
    Test {},
}

impl QueryMsg {
    pub fn get_validation_params(&self) -> (&HumanAddr, ViewingKey) {
        match self {
            Self::Balance { address, key } => (address, ViewingKey(key.clone())),
            Self::TransferHistory { address, key, .. } => (address, ViewingKey(key.clone())),
            _ => (panic!("lol")),
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    TokenInfo {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: Option<Uint128>,
    },
    ExchangeRate {
        rate: Uint128,
        denom: String,
    },
    Swap {
        result: Swap,
    },
    Allowance {
        spender: HumanAddr,
        owner: HumanAddr,
        allowance: Uint128,
        expiration: Option<u64>,
    },
    Balance {
        amount: Uint128,
    },
    TransferHistory {
        txs: Vec<Tx>,
    },

    ViewingKeyError {
        msg: String,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct CreateViewingKeyResponse {
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatusLevel {
    NormalRun,
    StopAllButWithdrawals,
    StopAll,
}

pub fn status_level_to_u8(status_level: ContractStatusLevel) -> u8 {
    match status_level {
        ContractStatusLevel::NormalRun => 0,
        ContractStatusLevel::StopAllButWithdrawals => 1,
        ContractStatusLevel::StopAll => 2,
    }
}

pub fn u8_to_status_level(status_level: u8) -> StdResult<ContractStatusLevel> {
    match status_level {
        0 => Ok(ContractStatusLevel::NormalRun),
        1 => Ok(ContractStatusLevel::StopAllButWithdrawals),
        2 => Ok(ContractStatusLevel::StopAll),
        _ => Err(StdError::generic_err("Invalid state level")),
    }
}

// Take a Vec<u8> and pad it up to a multiple of `block_size`, using spaces at the end.
pub fn space_pad(block_size: usize, message: &mut Vec<u8>) -> &mut Vec<u8> {
    let len = message.len();
    let surplus = len % block_size;
    if surplus == 0 {
        return message;
    }

    let missing = block_size - surplus;
    message.reserve(missing);
    message.extend(std::iter::repeat(b' ').take(missing));
    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{from_slice, StdResult};

    #[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum Something {
        Var { padding: Option<String> },
    }

    #[test]
    fn test_deserialization_of_missing_option_fields() -> StdResult<()> {
        let input = b"{ \"var\": {} }";
        let obj: Something = from_slice(input)?;
        assert_eq!(
            obj,
            Something::Var { padding: None },
            "unexpected value: {:?}",
            obj
        );
        Ok(())
    }
}
