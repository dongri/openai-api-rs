use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;

use crate::impl_builder_methods;
use crate::v1::common;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ToolChoiceType {
    None,
    Auto,
    ToolChoice { tool: Tool },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_tool_choice")]
    pub tool_choice: Option<ToolChoiceType>,
}

impl ChatCompletionRequest {
    pub fn new(model: String, messages: Vec<ChatCompletionMessage>) -> Self {
        Self {
            model,
            messages,
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
            seed: None,
            tools: None,
            tool_choice: None,
        }
    }
}

impl_builder_methods!(
    ChatCompletionRequest,
    temperature: f64,
    top_p: f64,
    n: i64,
    response_format: Value,
    stream: bool,
    stop: Vec<String>,
    max_tokens: i64,
    presence_penalty: f64,
    frequency_penalty: f64,
    logit_bias: HashMap<String, i32>,
    user: String,
    seed: i64,
    tools: Vec<Tool>,
    tool_choice: ToolChoiceType
);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MessageRole {
    user,
    system,
    assistant,
    function,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Content {
    Text(String),
    ImageUrl(Vec<ImageUrl>),
}

impl serde::Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Content::Text(ref text) => serializer.serialize_str(text),
            Content::ImageUrl(ref image_url) => image_url.serialize(serializer),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ContentType {
    text,
    image_url,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct ImageUrlType {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct ImageUrl {
    pub r#type: ContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrlType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessage {
    pub role: MessageRole,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionMessageForResponse {
    pub role: MessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionChoice {
    pub index: i64,
    pub message: ChatCompletionMessageForResponse,
    pub finish_reason: Option<FinishReason>,
    pub finish_details: Option<FinishDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: common::Usage,
    pub system_fingerprint: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: FunctionParameters,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum JSONSchemaType {
    Object,
    Number,
    String,
    Array,
    Null,
    Boolean,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub schema_type: JSONSchemaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<JSONSchemaDefine>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FinishReason {
    stop,
    length,
    content_filter,
    tool_calls,
    null,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub struct FinishDetails {
    pub r#type: FinishReason,
    pub stop: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCallFunction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

fn serialize_tool_choice<S>(
    value: &Option<ToolChoiceType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(ToolChoiceType::None) => serializer.serialize_str("none"),
        Some(ToolChoiceType::Auto) => serializer.serialize_str("auto"),
        Some(ToolChoiceType::ToolChoice { tool }) => {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("type", &tool.r#type)?;
            map.serialize_entry("function", &tool.function)?;
            map.end()
        }
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Tool {
    pub r#type: ToolType,
    pub function: Function,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    Function,
}
