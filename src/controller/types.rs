#![allow(non_snake_case)]

use axum::async_trait;
use axum::body::Bytes;
use axum::extract::multipart::Field;
use axum_typed_multipart::{
    FieldData, FieldMetadata, TryFromField, TryFromMultipart, TypedMultipartError,
};
use serde::Serialize;

#[derive(TryFromMultipart)]
pub struct DownloadRequest {
    pub fileName: String,
}

pub enum Algorithm {
    Aes,
    Rsa,
}

fn return_field_error() -> TypedMultipartError {
    TypedMultipartError::WrongFieldType {
        field_name: "algorithm".to_string(),
        wanted_type: "AES or RSA".to_string(),
    }
}

#[async_trait]
impl TryFromField for Algorithm {
    async fn try_from_field(field: Field<'_>) -> Result<Self, TypedMultipartError> {
        let alg_id: u8 = field
            .text()
            .await?
            .parse()
            .map_err(|_| return_field_error())?;
        match alg_id {
            0 => Ok(Algorithm::Aes),
            1 => Ok(Algorithm::Rsa),
            _ => Err(return_field_error()),
        }
    }
}

#[derive(TryFromMultipart)]
pub struct UploadRequest {
    pub algorithm: Option<Algorithm>,
    pub file: FieldData<Bytes>,
}

#[derive(Serialize)]
pub struct UploadData {
    name: String,
    mimetype: String,
    size: usize,
    time: usize,
}

impl UploadData {
    pub fn from_file(file: FieldMetadata, size: usize) -> UploadData {
        UploadData {
            name: file.name.unwrap(),
            mimetype: file.content_type.unwrap(),
            time: 0,
            size,
        }
    }
}

#[derive(Serialize)]
pub struct UploadResponse<'a> {
    pub status: bool,
    pub message: &'a str,
    pub data: UploadData,
}
