use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
pub struct CreateMessageRequest {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl CreateMessageRequest {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            role,
            content,
            attachments: None,
            metadata: None,
        }
    }
}

impl_builder_methods!(
    CreateMessageRequest,
    attachments: Vec<Attachment>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub file_id: Option<String>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: ContentText,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContentText {
    pub value: String,
    pub annotations: Vec<ContentTextAnnotations>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListMessage {
    pub object: String,
    pub data: Vec<MessageObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageFileObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub message_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListMessageFile {
    pub object: String,
    pub data: Vec<MessageFileObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ContentTextAnnotations {
    FileCitation(ContentTextAnnotationsFileCitationObject),
    FilePath(ContentTextAnnotationsFilePathObject),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContentTextAnnotationsFileCitationObject {
    pub text: String,
    pub file_citation: FileCitation,
    pub start_index: u32,
    pub end_index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileCitation {
    pub file_id: String,
    pub quote: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContentTextAnnotationsFilePathObject {
    pub text: String,
    pub file_path: FilePath,
    pub start_index: u32,
    pub end_index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FilePath {
    pub file_id: String,
}
