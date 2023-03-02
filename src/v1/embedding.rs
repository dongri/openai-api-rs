use serde::{Deserialize, Serialize};
use std::option::Option;

use crate::v1::common;

#[derive(Debug, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: i32,
    pub usage: common::Usage,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
}
