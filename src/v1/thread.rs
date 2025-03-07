use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
pub struct CreateThreadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ToolResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolResource {
    pub code_interpreter: Option<CodeInterpreter>,
    pub file_search: Option<FileSearch>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CodeInterpreter {
    pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSearch {
    pub vector_store_ids: Option<Vec<String>>,
    pub vector_stores: Option<VectorStores>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VectorStores {
    pub file_ids: Option<Vec<String>>,
    pub chunking_strategy: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateThreadRequest {
    pub fn new() -> Self {
        Self {
            messages: None,
            tool_resources: None,
            metadata: None,
        }
    }
}

impl Default for CreateThreadRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
    CreateThreadRequest,
    messages: Vec<Message>,
    tool_resources: ToolResource
);

#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ToolResource>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub thread_id: String,
    pub role: MessageRole,
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: ContentText,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentText {
    pub value: String,
    pub annotations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub file_id: String,
    pub tools: Vec<Tool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MessageRole {
    user,
    system,
    assistant,
    function,
}

#[derive(Debug, Serialize, Clone)]
pub struct ModifyThreadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyThreadRequest {
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl Default for ModifyThreadRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
    ModifyThreadRequest,
    metadata: HashMap<String, String>
);
