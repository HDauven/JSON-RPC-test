use axum::{routing::post, Router};
use axum_jrpc::{JrpcResult, JsonRpcExtractor, JsonRpcResponse};
use crate::rpc::types::*;

pub fn router() -> Router {
    Router::new().route("/rpc", post(handle_request))
}

async fn handle_request(value: JsonRpcExtractor) -> JrpcResult {
    let answer_id = value.get_answer_id();
    match value.method.as_str() {
        "get_block_height" => Ok(JsonRpcResponse::success(answer_id, 123456)),
        "get_balance" => {
            let _: GetBalanceRequest = value.parse_params()?;
            Ok(JsonRpcResponse::success(answer_id, GetBalanceResponse { balance: 1000 }))
        }
        "get_block" => {
            let request: GetBlockRequest = value.parse_params()?;
            Ok(JsonRpcResponse::success(answer_id, GetBlockResponse {
                block_hash: request.block_hash,
                transactions: vec!["tx1".into(), "tx2".into()],
                timestamp: 1617181723,
            }))
        }
        "get_transaction" => {
            let request: GetTransactionRequest = value.parse_params()?;
            Ok(JsonRpcResponse::success(answer_id, GetTransactionResponse {
                tx_hash: request.tx_hash,
                from: "address1".into(),
                to: "address2".into(),
                amount: 500,
                status: "confirmed".into(),
            }))
        }
        method => Ok(value.method_not_found(method)),
    }
}
