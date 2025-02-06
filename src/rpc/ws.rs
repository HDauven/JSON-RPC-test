use axum::{routing::get, extract::ws::{WebSocket, WebSocketUpgrade, Message}, Router, response::Response};
use axum_jrpc::{JsonRpcResponse, error::{JsonRpcError, JsonRpcErrorReason}};
use serde_json::Value;
use crate::rpc::types::*;

pub fn router() -> Router {
    Router::new().route("/ws", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_request)
}

async fn handle_request(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(text)) = msg {
            if let Ok(value) = serde_json::from_str::<Value>(&text) {
                let method = value.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let id = value.get("id").cloned().unwrap_or(Value::Null);
                let params = value.get("params").cloned().unwrap_or(Value::Null);
                
                let answer_id = match &id {
                    Value::String(s) => axum_jrpc::Id::from(s.clone()),
                    Value::Number(n) => axum_jrpc::Id::from(n.as_i64().unwrap_or(0)),
                    _ => axum_jrpc::Id::None(()),
                };
                
                let response = match method {
                    "get_block_height" => JsonRpcResponse::success(answer_id, 123456),
                    "get_balance" => {
                        match serde_json::from_value::<GetBalanceRequest>(params) {
                            Ok(_) => JsonRpcResponse::success(answer_id, GetBalanceResponse { balance: 1000 }),
                            Err(_) => JsonRpcResponse::error(answer_id, JsonRpcError::new(JsonRpcErrorReason::InvalidParams, "Invalid parameters".to_string(), Value::default())),
                        }
                    }
                    "get_block" => {
                        match serde_json::from_value::<GetBlockRequest>(params) {
                            Ok(req) => JsonRpcResponse::success(answer_id, GetBlockResponse {
                                block_hash: req.block_hash,
                                transactions: vec!["tx1".into(), "tx2".into()],
                                timestamp: 1617181723,
                            }),
                            Err(_) => JsonRpcResponse::error(answer_id, JsonRpcError::new(JsonRpcErrorReason::InvalidParams, "Invalid parameters".to_string(), Value::default())),
                        }
                    }
                    "get_transaction" => {
                        match serde_json::from_value::<GetTransactionRequest>(params) {
                            Ok(req) => JsonRpcResponse::success(answer_id, GetTransactionResponse {
                                tx_hash: req.tx_hash,
                                from: "address1".into(),
                                to: "address2".into(),
                                amount: 500,
                                status: "confirmed".into(),
                            }),
                            Err(_) => JsonRpcResponse::error(answer_id, JsonRpcError::new(JsonRpcErrorReason::InvalidParams, "Invalid parameters".to_string(), Value::default())),
                        }
                    }
                    _ => JsonRpcResponse::error(answer_id, JsonRpcError::new(JsonRpcErrorReason::MethodNotFound, "Method not found".to_string(), Value::default())),
                };
                
                let response_text = serde_json::to_string(&response).unwrap();
                if socket.send(Message::Text(response_text.into())).await.is_err() {
                    tracing::error!("Failed to send WebSocket response");
                    return;
                }
            }
        }
    }
    tracing::info!("WebSocket connection closed");
}
