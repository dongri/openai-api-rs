use crate::v1::types::Tools;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

// pub mod responses_stream;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateResponseRequest {
    // background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    // conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Value>,

    // include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

    // input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Value>,

    // instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    // max_output_tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    // max_tool_calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u32>,

    // metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,

    // model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    // parallel_tool_calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    // previous_response_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    // prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Value>,

    // prompt_cache_key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    // reasoning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Value>,

    // safety_identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,

    // service_tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,

    // store
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    // stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    // stream_options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<Value>,

    // temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    // text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,

    // tool_choice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,

    // tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tools>>,

    // top_logprobs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,

    // top_p
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    // truncation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,

    // user (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    // Future-proof
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl CreateResponseRequest {
    pub fn new() -> Self {
        Self {
            background: None,
            conversation: None,
            include: None,
            input: None,
            instructions: None,
            max_output_tokens: None,
            max_tool_calls: None,
            metadata: None,
            model: None,
            parallel_tool_calls: None,
            previous_response_id: None,
            prompt: None,
            prompt_cache_key: None,
            reasoning: None,
            safety_identifier: None,
            service_tier: None,
            store: None,
            stream: None,
            stream_options: None,
            temperature: None,
            text: None,
            tool_choice: None,
            tools: None,
            top_logprobs: None,
            top_p: None,
            truncation: None,
            user: None,
            extra: BTreeMap::new(),
        }
    }
}

impl Default for CreateResponseRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseObject {
    pub id: String,
    pub object: String,

    // Core
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    // Output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio: Option<Value>,

    // Control / reasons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,

    // Tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Value>,

    // Misc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,

    // Errors / details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,

    // Future-proof
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListResponses {
    pub object: String,
    pub data: Vec<ResponseObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    pub has_more: bool,
}

// Get input token counts (POST /v1/responses/input_tokens)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountTokensRequest {
    // conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Value>,

    // input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Value>,

    // instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    // model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    // parallel_tool_calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    // previous_response_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    // reasoning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Value>,

    // text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,

    // tool_choice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,

    // tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tools>>,

    // truncation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,

    // Future-proof
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl CountTokensRequest {
    pub fn new() -> Self {
        Self {
            conversation: None,
            input: None,
            instructions: None,
            model: None,
            parallel_tool_calls: None,
            previous_response_id: None,
            reasoning: None,
            text: None,
            tool_choice: None,
            tools: None,
            truncation: None,
            extra: BTreeMap::new(),
        }
    }
}

impl Default for CountTokensRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountTokensResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u32>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
