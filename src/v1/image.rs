use serde::{Deserialize, Serialize};
use std::option::Option;

use crate::impl_builder_methods;

#[derive(Debug, Deserialize)]
pub struct ImageData {
    pub url: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ImageGenerationRequest {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            model: None,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    ImageGenerationRequest,
    model: String,
    n: i32,
    size: String,
    response_format: String,
    user: String
);

#[derive(Debug, Deserialize)]
pub struct ImageGenerationResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageEditRequest {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ImageEditRequest {
    pub fn new(image: String, prompt: String) -> Self {
        Self {
            image,
            prompt,
            mask: None,
            model: None,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    ImageEditRequest,
    mask: String,
    model: String,
    n: i32,
    size: String,
    response_format: String,
    user: String
);

#[derive(Debug, Deserialize)]
pub struct ImageEditResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageVariationRequest {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ImageVariationRequest {
    pub fn new(image: String) -> Self {
        Self {
            image,
            model: None,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    ImageVariationRequest,
    model: String,
    n: i32,
    size: String,
    response_format: String,
    user: String
);

#[derive(Debug, Deserialize)]
pub struct ImageVariationResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}
