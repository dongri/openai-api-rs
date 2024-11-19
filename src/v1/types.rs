use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: FunctionParameters,
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
    pub function: Function,
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
