use serde::{Serialize, Deserialize};
use std::option::Option;

use crate::v1::common;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct EditChoice{
	pub text:  String,
	pub index: i32,
}

#[derive(Debug, Deserialize)]
pub struct EditResponse {
    pub object:  String,
    pub created: i64,
    pub usage:   common::Usage,
    pub choices: Vec<EditChoice>,
}
