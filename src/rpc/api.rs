use yerpc::rpc;
use crate::rpc::types::*;

#[derive(Clone)]
pub struct Api;

#[rpc(all_positional, openrpc_outdir = "./")]
impl Api {
    pub async fn get_block_height(&self) -> u64 {
        123456
    }

    pub async fn get_balance(&self, _address: String) -> GetBalanceResponse {
        GetBalanceResponse { balance: 1000 }
    }

    pub async fn get_block(&self, block_hash: String) -> GetBlockResponse {
        GetBlockResponse {
            block_hash,
            transactions: vec!["tx1".into(), "tx2".into()],
            timestamp: 1617181723,
        }
    }

    pub async fn get_transaction(&self, tx_hash: String) -> GetTransactionResponse {
        GetTransactionResponse {
            tx_hash,
            from: "address1".into(),
            to: "address2".into(),
            amount: 500,
            status: "confirmed".into(),
        }
    }
}
