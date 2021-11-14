use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub incentivized_token: SecretContract,
    pub profit_token: SecretContract,
    pub total_shares: u128,
    pub viewing_key: String,
    pub per_share_scaled: String,
    pub residue: u128,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecretContract {
    pub address: HumanAddr,
    pub contract_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub debt: String,
    pub shares: u128,
}
