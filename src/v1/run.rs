use super::thread::CreateThreadRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
pub struct CreateRunRequest {
    assistant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>, // 1: json!("auto"), 2: json!({"type": "json_object"})
}

impl CreateRunRequest {
    pub fn new(assistant_id: String) -> Self {
        Self {
            assistant_id,
            model: None,
            instructions: None,
            tools: None,
            metadata: None,
            response_format: None,
        }
    }
}

impl_builder_methods!(
    CreateRunRequest,
    model: String,
    instructions: String,
    tools: Vec<HashMap<String, String>>,
    metadata: HashMap<String, String>,
    response_format: Value
);

#[derive(Debug, Serialize, Clone)]
pub struct ModifyRunRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyRunRequest {
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl Default for ModifyRunRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
    ModifyRunRequest,
    metadata: HashMap<String, String>
);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LastError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub thread_id: String,
    pub assistant_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_action: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<LastError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    pub model: String,
    pub instructions: Option<String>,
    pub tools: Vec<HashMap<String, String>>,
    pub metadata: HashMap<String, String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListRun {
    pub object: String,
    pub data: Vec<RunObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateThreadAndRunRequest {
    pub assistant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<CreateThreadRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RunStepObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub assistant_id: String,
    pub thread_id: String,
    pub run_id: String,
    #[serde(rename = "type")]
    pub run_step_type: String,
    pub status: String,
    pub step_details: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<LastError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    pub metadata: HashMap<String, String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListRunStep {
    pub object: String,
    pub data: Vec<RunStepObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
    pub headers: Option<HashMap<String, String>>,
}
