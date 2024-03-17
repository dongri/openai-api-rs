use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::option::Option;

use crate::impl_builder_methods;
use crate::v1::common;

#[derive(Debug, Serialize, Clone)]
pub struct EditRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

impl EditRequest {
    pub fn new(model: String, instruction: String) -> Self {
        Self {
            model,
            instruction,
            input: None,
            n: None,
            temperature: None,
            top_p: None,
        }
    }
}

impl_builder_methods!(
    EditRequest,
    input: String,
    n: i32,
    temperature: f32,
    top_p: f32
);

#[derive(Debug, Deserialize, Serialize)]
pub struct EditChoice {
    pub text: String,
    pub index: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditResponse {
    pub object: String,
    pub created: i64,
    pub usage: common::Usage,
    pub choices: Vec<EditChoice>,
    pub headers: Option<HashMap<String, String>>,
}
