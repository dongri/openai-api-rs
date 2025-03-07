use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types;
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
    pub tool_resources: Option<ToolResource>,
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
            tool_resources: None,
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
    tool_resources: ToolResource,
    metadata: HashMap<String, String>
);

#[derive(Debug, Deserialize, Serialize)]
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
    pub tools: Vec<types::Tools>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ToolResource>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_interpreter: Option<CodeInterpreter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<FileSearch>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CodeInterpreter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_store_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_stores: Option<VectorStores>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VectorStores {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeletionStatus {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListAssistant {
    pub object: String,
    pub data: Vec<AssistantObject>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AssistantFileRequest {
    pub file_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssistantFileObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub assistant_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListAssistantFile {
    pub object: String,
    pub data: Vec<AssistantFileObject>,
}
