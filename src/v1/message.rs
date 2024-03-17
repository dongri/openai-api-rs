use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
pub struct CreateMessageRequest {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateMessageRequest {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            role,
            content,
            file_ids: None,
            metadata: None,
        }
    }
}

impl_builder_methods!(
    CreateMessageRequest,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>
);

#[derive(Debug, Serialize, Clone)]
pub struct ModifyMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyMessageRequest {
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl Default for ModifyMessageRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
    ModifyMessageRequest,
    metadata: HashMap<String, String>
);

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageObject {
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
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MessageRole {
    user,
    system,
    assistant,
    function,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: ContentText,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContentText {
    pub value: String,
    pub annotations: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListMessage {
    pub object: String,
    pub data: Vec<MessageObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageFileObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub message_id: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListMessageFile {
    pub object: String,
    pub data: Vec<MessageFileObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
    pub headers: Option<HashMap<String, String>>,
}
