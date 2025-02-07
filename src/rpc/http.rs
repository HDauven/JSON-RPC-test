use std::sync::Arc;

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use serde_json::Value;
use crate::rpc::api::Api;

pub fn router() -> Router {
    Router::new().route("/rpc", post(handle_request))
}

async fn handle_request(
    Extension(api): Extension<Arc<Api>>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let answer_id = payload.get("id").cloned().unwrap_or(Value::Null);
    let method = payload.get("method").and_then(|m| m.as_str()).unwrap_or("");

    let response = match method {
        "get_block_height" => serde_json::to_value(api.get_block_height().await).unwrap(),
        "get_balance" => {
            let params = payload.get("params").and_then(|p| p.get("address")).and_then(|a| a.as_str()).unwrap_or("");
            serde_json::to_value(api.get_balance(params.to_string()).await).unwrap()
        }
        "get_block" => {
            let params = payload.get("params").and_then(|p| p.get("block_hash")).and_then(|b| b.as_str()).unwrap_or("");
            serde_json::to_value(api.get_block(params.to_string()).await).unwrap()
        }
        "get_transaction" => {
            let params = payload.get("params").and_then(|p| p.get("tx_hash")).and_then(|t| t.as_str()).unwrap_or("");
            serde_json::to_value(api.get_transaction(params.to_string()).await).unwrap()
        }
        _ => serde_json::json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32601,
                "message": "Method not found"
            },
            "id": answer_id
        }),
    };

    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "result": response,
        "id": answer_id
    }))
}
