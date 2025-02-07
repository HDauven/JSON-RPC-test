use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetBalanceRequest {
    pub address: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetBlockRequest {
    pub block_hash: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetTransactionRequest {
    pub tx_hash: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetBalanceResponse {
    pub balance: u64,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetBlockResponse {
    pub block_hash: String,
    pub transactions: Vec<String>,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetTransactionResponse {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub status: String,
}
