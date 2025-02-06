use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBalanceRequest {
    pub address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBlockRequest {
    pub block_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTransactionRequest {
    pub tx_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBalanceResponse {
    pub balance: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBlockResponse {
    pub block_hash: String,
    pub transactions: Vec<String>,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTransactionResponse {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub status: String,
}
