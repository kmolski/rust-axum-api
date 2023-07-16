use axum::body::StreamBody;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::post, Router};
use axum_typed_multipart::TypedMultipart;
use tokio_util::io::ReaderStream;

use crate::controller::types::*;
use crate::file_manager::FileManager;

mod types;

async fn download_file(
    State(file_manager): State<FileManager>,
    TypedMultipart(body): TypedMultipart<DownloadRequest>,
) -> impl IntoResponse {
    let Ok(file) = file_manager.open_file(&body.fileName).await else {
        return Err((StatusCode::NOT_FOUND, format!("File not found: {}", body.fileName)));
    };

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok(body)
}

async fn upload_file(
    State(file_manager): State<FileManager>,
    TypedMultipart(body): TypedMultipart<UploadRequest>,
) -> impl IntoResponse {
    let file = body.file;
    let name = file.metadata.file_name.clone().unwrap();
    let data = file.contents;
    if file_manager.save_file(&name, &data).await.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create file"));
    }

    Ok(Json(UploadResponse {
        status: true,
        message: "File is uploaded",
        data: UploadData::from_file(file.metadata, data.len()),
    }))
}

async fn encrypt_sync(
    State(file_manager): State<FileManager>,
    TypedMultipart(body): TypedMultipart<UploadRequest>,
) -> impl IntoResponse {
    let file = body.file;
    let name = file.metadata.file_name.clone().unwrap();
    let data = file.contents;
    if file_manager.save_file(&name, &data).await.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create file"));
    }

    Ok(Json(UploadResponse {
        status: true,
        message: "File is uploaded",
        data: UploadData::from_file(file.metadata, data.len()),
    }))
}

pub fn routes_streaming_encryption(file_manager: FileManager) -> Router {
    Router::new()
    //.route("/upload/encryption-stream", post(upload_encrypted_streaming))
    //.route("/upload/decryption-stream", post(download_encrypted_streaming))
    //.with_state(file_manager)
}

pub fn routes_sync_encryption(file_manager: FileManager) -> Router {
    Router::new()
        .route("/upload/encryption", post(encrypt_sync))
        //.route("/upload/decryption", post(decrypt_sync))
        .with_state(file_manager)
}

pub fn routes_no_encryption(file_manager: FileManager) -> Router {
    Router::new()
        .route("/upload/no-encryption", post(upload_file))
        .route("/download", post(download_file))
        .with_state(file_manager)
}
