use serde::{Deserialize, Serialize};

use crate::v1::common;

pub const GPT3_5_TURBO: &str = "gpt-3.5-turbo";
pub const GPT3_5_TURBO_0301: &str = "gpt-3.5-turbo-0301";

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MessageRole {
    user,
    system,
    assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: i64,
    pub message: ChatCompletionMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: common::Usage,
}
