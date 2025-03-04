use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileData {
    pub id: String,
    pub object: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileListResponse {
    pub object: String,
    pub data: Vec<FileData>,
}

#[derive(Debug, Serialize)]
pub struct FileUploadRequest {
    pub file: String,
    pub purpose: String,
}

impl FileUploadRequest {
    pub fn new(file: String, purpose: String) -> Self {
        Self { file, purpose }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileUploadResponse {
    pub id: String,
    pub object: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Serialize)]
pub struct FileDeleteRequest {
    pub file_id: String,
}

impl FileDeleteRequest {
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileDeleteResponse {
    pub id: String,
    pub object: String,
    pub delete: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileRetrieveResponse {
    pub id: String,
    pub object: String,
    pub bytes: i32,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}
