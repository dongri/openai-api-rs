use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Debug, Deserialize)]
pub struct ImageData {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImageGenerationResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Serialize)]
pub struct ImageEditRequest {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImageEditResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Serialize)]
pub struct ImageVariationRequest {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImageVariationResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}
