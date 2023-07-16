use std::env;
use std::net::SocketAddr;

use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::{Router, Server};

use crate::controller::*;
use crate::file_manager::*;

mod controller;
mod crypto;
mod file_manager;

#[tokio::main]
async fn main() {
    let base_dir_val = env::var("UPLOAD_BASE_DIR");
    let base_dir = base_dir_val.as_deref().unwrap_or("./example_data/");
    eprintln!("Serving files from `{}`", base_dir);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("Example app listening on port {}", address.port());

    let file_manager = FileManager::new(base_dir);
    let router = Router::new()
        .merge(routes_streaming_encryption(file_manager.clone()))
        .merge(routes_sync_encryption(file_manager.clone()))
        .merge(routes_no_encryption(file_manager.clone()))
        .route("/", get(|| async { "Hello World!" }))
        .layer(DefaultBodyLimit::max(1024_usize.pow(3))); // 1 gigabyte
    Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
