use crate::state::SecretContract;
use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProfitDistributorBInitMsg {
    pub incentivized_token: SecretContract,
    pub profit_token: SecretContract,
    pub viewing_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfitDistributorBHandleMsg {
    Receive {
        sender: HumanAddr,
        from: HumanAddr,
        amount: Uint128,
        msg: Binary,
    },
    Withdraw {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ProfitDistributorBHandleAnswer {
    Withdraw {
        status: ProfitDistributorBResponseStatus,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfitDistributorBQueryMsg {
    Config {},
    ClaimableProfit { user_address: HumanAddr },
    User { user_address: HumanAddr },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ProfitDistributorBQueryAnswer {
    ClaimableProfit {
        amount: Uint128,
    },
    Config {
        incentivized_token: SecretContract,
        per_share_scaled: String,
        residue: Uint128,
        profit_token: SecretContract,
        total_shares: Uint128,
        viewing_key: String,
    },
    User {
        debt: String,
        shares: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    Deposit {},
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveAnswer {
    Deposit {
        status: ProfitDistributorBResponseStatus,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ProfitDistributorBResponseStatus {
    Success,
}
