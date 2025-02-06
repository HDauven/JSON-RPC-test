use axum::{routing::get, extract::ws::WebSocketUpgrade, Router, response::Response};
use yerpc::{rpc, RpcClient, RpcSession};
use yerpc::axum::handle_ws_rpc;

use crate::rpc::types::*;

pub fn router() -> Router {
    Router::new().route("/ws", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    let api = Api;
    let (client, out_channel) = RpcClient::new();
    let session = RpcSession::new(client, api);
    handle_ws_rpc(ws, out_channel, session).await
}

struct Api;

#[rpc(all_positional, openrpc_outdir = "./")]
impl Api {
    async fn get_block_height(&self) -> u64 {
        123456
    }

    async fn get_balance(&self, _address: String) -> GetBalanceResponse {
        GetBalanceResponse { balance: 1000 }
    }

    async fn get_block(&self, block_hash: String) -> GetBlockResponse {
        GetBlockResponse {
            block_hash,
            transactions: vec!["tx1".into(), "tx2".into()],
            timestamp: 1617181723,
        }
    }

    async fn get_transaction(&self, tx_hash: String) -> GetTransactionResponse {
        GetTransactionResponse {
            tx_hash,
            from: "address1".into(),
            to: "address2".into(),
            amount: 500,
            status: "confirmed".into(),
        }
    }
}
