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
    pub tools: Vec<Tools>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ToolResource>,
    pub metadata: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Tools {
    CodeInterpreter,
    FileSearch(ToolsFileSearch),
    Function(ToolsFunction),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolsFileSearch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<ToolsFileSearchObject>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolsFunction {
    pub function: types::Function,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolsFileSearchObject {
    pub max_num_results: Option<u8>,
    pub ranking_options: Option<FileSearchRankingOptions>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSearchRankingOptions {
    pub ranker: Option<FileSearchRanker>,
    pub score_threshold: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FileSearchRanker {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default_2024_08_21")]
    Default2024_08_21,
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
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListAssistant {
    pub object: String,
    pub data: Vec<AssistantObject>,
    pub headers: Option<HashMap<String, String>>,
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
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListAssistantFile {
    pub object: String,
    pub data: Vec<AssistantFileObject>,
    pub headers: Option<HashMap<String, String>>,
}
