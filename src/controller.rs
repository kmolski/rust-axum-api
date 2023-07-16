#![allow(non_snake_case)]

use axum::{Router, routing::post};
use axum::body::StreamBody;
use axum::extract::{Json, Multipart, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use serde::Serialize;
use tokio_util::io::ReaderStream;

use crate::file_manager::FileManager;

#[derive(TryFromMultipart)]
struct DownloadRequest {
    fileName: String,
}

#[derive(Serialize)]
struct UploadData {
    name: String,
    mimetype: String,
    size: usize,
    time: usize,
}

#[derive(Serialize)]
struct UploadResponse<'a> {
    status: bool,
    message: &'a str,
    data: UploadData,
}

async fn download_file(
    State(file_manager): State<FileManager>,
    TypedMultipart(body): TypedMultipart<DownloadRequest>
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
    mut body: Multipart
) -> impl IntoResponse {

    while let Some(field) = body.next_field().await.unwrap() {
        if !field.name().is_some_and(|n| n == "file") {
            continue;
        }

        let name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        if file_manager.save_file(&name, &data).await.is_err() {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create file"));
        }

        return Ok(Json(UploadResponse {
            status: true,
            message: "File is uploaded",
            data: UploadData {
                name,
                mimetype: content_type,
                size: data.len(),
                time: 0,
            },
        }));
    }

    Err((
        StatusCode::BAD_REQUEST,
        "Multipart must contain a `file` field",
    ))
}

async fn encrypt_sync(
    State(file_manager): State<FileManager>,
    mut body: Multipart
) -> impl IntoResponse {

    while let Some(field) = body.next_field().await.unwrap() {
        if !field.name().is_some_and(|n| n == "file") {
            continue;
        }

        let name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        if let Err(_) = file_manager.save_file(&name, &data).await {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create file"));
        }

        return Ok(Json(UploadResponse {
            status: true,
            message: "File is uploaded",
            data: UploadData {
                name,
                mimetype: content_type,
                size: data.len(),
                time: 0,
            },
        }));
    }

    Err((
        StatusCode::BAD_REQUEST,
        "Multipart must contain a `file` field",
    ))
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
