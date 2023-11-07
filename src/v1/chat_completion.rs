use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

use crate::impl_builder_methods;
use crate::v1::common;

// https://platform.openai.com/docs/models/gpt-3-5
pub const GPT3_5_TURBO_1106: &str = "gpt-3.5-turbo-1106";
pub const GPT3_5_TURBO: &str = "gpt-3.5-turbo";
pub const GPT3_5_TURBO_16K: &str = "gpt-3.5-turbo-16k";
pub const GPT3_5_TURBO_INSTRUCT: &str = "gpt-3.5-turbo-instruct";
// - legacy
pub const GPT3_5_TURBO_0613: &str = "gpt-3.5-turbo-0613";
pub const GPT3_5_TURBO_16K_0613: &str = "gpt-3.5-turbo-16k-0613";
pub const GPT3_5_TURBO_0301: &str = "gpt-3.5-turbo-0301";

// https://platform.openai.com/docs/models/gpt-4-and-gpt-4-turbo
pub const GPT4_1106_PREVIEW: &str = "gpt-4-1106-preview";
pub const GPT4_VISION_PREVIEW: &str = "gpt-4-vision-preview";
pub const GPT4: &str = "gpt-4";
pub const GPT4_32K: &str = "gpt-4-32k";
pub const GPT4_0613: &str = "gpt-4-0613";
pub const GPT4_32K_0613: &str = "gpt-4-32k-0613";
// - legacy
pub const GPT4_0314: &str = "gpt-4-0314";
pub const GPT4_32K_0314: &str = "gpt-4-32k-0314";

#[derive(Debug, Serialize)]
pub enum FunctionCallType {
    None,
    Auto,
    Function { name: String },
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_function_call")]
    pub function_call: Option<FunctionCallType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ChatCompletionRequest {
    pub fn new(model: String, messages: Vec<ChatCompletionMessage>) -> Self {
        Self {
            model,
            messages,
            functions: None,
            function_call: None,
            temperature: None,
            top_p: None,
            stream: None,
            n: None,
            response_format: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }
}

impl_builder_methods!(
    ChatCompletionRequest,
    functions: Vec<Function>,
    function_call: FunctionCallType,
    temperature: f64,
    top_p: f64,
    n: i64,
    response_format: String,
    stream: bool,
    stop: Vec<String>,
    max_tokens: i64,
    presence_penalty: f64,
    frequency_penalty: f64,
    logit_bias: HashMap<String, i32>,
    user: String
);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum MessageRole {
    user,
    system,
    assistant,
    function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionMessage {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionMessageForResponse {
    pub role: MessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub index: i64,
    pub message: ChatCompletionMessageForResponse,
    pub finish_reason: FinishReason,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: FunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JSONSchemaType {
    Object,
    Number,
    String,
    Array,
    Null,
    Boolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONSchemaDefine {
    #[serde(rename = "type")]
    pub schema_type: Option<JSONSchemaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<JSONSchemaDefine>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub schema_type: JSONSchemaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum FinishReason {
    stop,
    length,
    function_call,
    content_filter,
    null,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

fn serialize_function_call<S>(
    value: &Option<FunctionCallType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(FunctionCallType::None) => serializer.serialize_str("none"),
        Some(FunctionCallType::Auto) => serializer.serialize_str("auto"),
        Some(FunctionCallType::Function { name }) => {
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_entry("name", name)?;
            map.end()
        }
        None => serializer.serialize_none(),
    }
}
