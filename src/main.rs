use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::net::SocketAddr;
use std::path::Path;

use axum::{Router, Server};
use axum::extract::{DefaultBodyLimit, FromRef};
use axum::routing::get;

use crate::controller::*;
use crate::crypto::KeyManager;
use crate::file_manager::FileManager;

mod controller;
mod crypto;
mod file_manager;

#[derive(Clone, FromRef)]
pub struct AppState {
    file_manager: FileManager,
    key_manager: KeyManager,
}

fn read_key_file(path: impl AsRef<Path>) -> Result<String, Error> {
    let mut buf = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

#[tokio::main]
async fn main() {
    let base_dir_val = env::var("UPLOAD_BASE_DIR");
    let base_dir = base_dir_val.as_deref().unwrap_or("./example_data/");
    eprintln!("Serving files from `{}`", base_dir);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("Example app listening on port {}", address.port());

    let aes_key_val = env::var("CRYPT_AES_KEY").expect("Error: No AES key given");
    let rsa_pub_path = env::var("RSA_PUB_PEM");
    let rsa_priv_path = env::var("RSA_PRIV_PEM");
    let rsa_pub = read_key_file(rsa_pub_path.as_deref().unwrap_or("./public.pem"))
        .expect("Could not read public key");
    let rsa_priv = read_key_file(rsa_priv_path.as_deref().unwrap_or("./private.pem"))
        .expect("Could not read private key");

    let file_manager = FileManager::new(base_dir);
    let key_manager = KeyManager::new(aes_key_val, rsa_pub, rsa_priv);
    let app_state = AppState {
        file_manager,
        key_manager,
    };

    let router = Router::new()
        .merge(routes_streaming_encryption(app_state.clone()))
        .merge(routes_sync_encryption(app_state.clone()))
        .merge(routes_no_encryption(app_state.clone()))
        .route("/", get(|| async { "Hello World!" }))
        .layer(DefaultBodyLimit::max(1024_usize.pow(3))); // 1 gigabyte
    Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
