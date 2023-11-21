use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
pub struct AssistantRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl AssistantRequest {
    pub fn new(model: String) -> Self {
        Self {
            model,
            name: None,
            description: None,
            instructions: None,
            tools: None,
            file_ids: None,
            metadata: None,
        }
    }
}

impl_builder_methods!(
    AssistantRequest,
    name: String,
    description: String,
    instructions: String,
    tools: Vec<HashMap<String, String>>,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>
);

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    pub tools: Vec<HashMap<String, String>>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionStatus {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAssistant {
    pub object: String,
    pub data: Vec<AssistantObject>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AssistantFileRequest {
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantFileObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub assistant_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAssistantFile {
    pub object: String,
    pub data: Vec<AssistantFileObject>,
}
