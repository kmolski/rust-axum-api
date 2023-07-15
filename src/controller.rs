#![allow(non_snake_case)]

use axum::body::StreamBody;
use axum::extract::{Json, Multipart};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::post, Router};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use serde::Serialize;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

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

async fn download_file(TypedMultipart(body): TypedMultipart<DownloadRequest>) -> impl IntoResponse {
    let mut path = PathBuf::from("./example_data/");
    path.push(&body.fileName);

    let Ok(file) = tokio::fs::File::open(path).await else {
        return Err((StatusCode::NOT_FOUND, format!("File not found: {}", body.fileName)));
    };
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok(body)
}

async fn upload_file(mut body: Multipart) -> impl IntoResponse {
    while let Some(field) = body.next_field().await.unwrap() {
        if !field.name().is_some_and(|n| n == "file") {
            continue;
        }

        let name = field.file_name().unwrap().to_string();
        let mut path = PathBuf::from("./example_data/");
        path.push(&name);

        let Ok(mut file) = tokio::fs::File::create(path).await else {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create file"));
        };

        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        file.write_all(&data).await.unwrap();

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

pub fn routes_streaming_encryption() -> Router {
    Router::new()
    //.route("/upload/encryption-stream", post(upload_encrypted_streaming))
    //.route("/upload/decryption-stream", post(download_encrypted_streaming))
}

pub fn routes_sync_encryption() -> Router {
    Router::new()
    //.route("/upload/encryption", post(upload_encrypted))
    //.route("/upload/decryption", post(download_encrypted))
}

pub fn routes_no_encryption() -> Router {
    Router::new()
        .route("/upload/no-encryption", post(upload_file))
        .route("/download", post(download_file))
}
