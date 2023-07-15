mod controller;

use std::env;
use std::net::SocketAddr;

use crate::controller::*;
use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::{Router, Server};

mod crypto {}

mod file_manager {
    use std::path::Path;

    trait FileStore {}

    trait FileLoad {}

    struct FileManager {
        base_dir: Path,
    }

    impl FileManager {}
}

#[tokio::main]
async fn main() {
    let base_dir = env::var("UPLOAD_BASE_DIR").unwrap_or("./example_data/".to_string());
    eprintln!("Serving files from `{}`", base_dir);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("Example app listening on port {}", address.port());

    let router = Router::new()
        .merge(routes_streaming_encryption())
        .merge(routes_sync_encryption())
        .merge(routes_no_encryption())
        .route("/", get(|| async { "Hello World!" }))
        .layer(DefaultBodyLimit::max(1024_usize.pow(3))); // 1 gigabyte
    Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
