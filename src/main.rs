use std::net::SocketAddr;

use axum::{Router, Server};
use axum::response::Html;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(|| async { Html("Hello World!") }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await;
}
