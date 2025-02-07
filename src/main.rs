use axum::{Router, Extension};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod rpc;
use rpc::http::router as http_router;
use rpc::ws::router as ws_router;
use rpc::api::Api;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api = Arc::new(Api);

    let router = Router::new()
        .merge(http_router())
        .merge(ws_router())
        .layer(Extension(api));

    tracing::debug!("Listening on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000".parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
