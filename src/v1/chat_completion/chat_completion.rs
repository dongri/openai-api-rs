use crate::v1::chat_completion::{ChatCompletionChoice, Reasoning, Tool, ToolChoiceType};
use crate::v1::common;
use crate::{
    impl_builder_methods,
    v1::chat_completion::{serialize_tool_choice, ChatCompletionMessage},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_tool_choice")]
    pub tool_choice: Option<ToolChoiceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    /// Optional list of transforms to apply to the chat completion request.
    ///
    /// Transforms allow modifying the request before it's sent to the API,
    /// enabling features like prompt rewriting, content filtering, or other
    /// preprocessing steps. When None, no transforms are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<String>>,
}

impl ChatCompletionRequest {
    pub fn new(model: String, messages: Vec<ChatCompletionMessage>) -> Self {
        Self {
            model,
            messages,
            temperature: None,
            top_p: None,
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
            parallel_tool_calls: None,
            tool_choice: None,
            reasoning: None,
            transforms: None,
        }
    }
}

impl_builder_methods!(
    ChatCompletionRequest,
    temperature: f64,
    top_p: f64,
    n: i64,
    response_format: Value,
    stop: Vec<String>,
    max_tokens: i64,
    presence_penalty: f64,
    frequency_penalty: f64,
    logit_bias: HashMap<String, i32>,
    user: String,
    seed: i64,
    tools: Vec<Tool>,
    parallel_tool_calls: bool,
    tool_choice: ToolChoiceType,
    reasoning: Reasoning,
    transforms: Vec<String>
);

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: common::Usage,
    pub system_fingerprint: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::v1::chat_completion::{ReasoningEffort, ReasoningMode};

    use super::*;
    use serde_json::json;

    #[test]
    fn test_reasoning_effort_serialization() {
        let reasoning = Reasoning {
            mode: Some(ReasoningMode::Effort {
                effort: ReasoningEffort::High,
            }),
            exclude: Some(false),
            enabled: None,
        };

        let serialized = serde_json::to_value(&reasoning).unwrap();
        let expected = json!({
            "effort": "high",
            "exclude": false
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_reasoning_max_tokens_serialization() {
        let reasoning = Reasoning {
            mode: Some(ReasoningMode::MaxTokens { max_tokens: 2000 }),
            exclude: None,
            enabled: Some(true),
        };

        let serialized = serde_json::to_value(&reasoning).unwrap();
        let expected = json!({
            "max_tokens": 2000,
            "enabled": true
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_reasoning_deserialization() {
        let json_str = r#"{"effort": "medium", "exclude": true}"#;
        let reasoning: Reasoning = serde_json::from_str(json_str).unwrap();

        match reasoning.mode {
            Some(ReasoningMode::Effort { effort }) => {
                assert_eq!(effort, ReasoningEffort::Medium);
            }
            _ => panic!("Expected effort mode"),
        }
        assert_eq!(reasoning.exclude, Some(true));
    }

    #[test]
    fn test_chat_completion_request_with_reasoning() {
        let mut req = ChatCompletionRequest::new("gpt-4".to_string(), vec![]);

        req.reasoning = Some(Reasoning {
            mode: Some(ReasoningMode::Effort {
                effort: ReasoningEffort::Low,
            }),
            exclude: None,
            enabled: None,
        });

        let serialized = serde_json::to_value(&req).unwrap();
        assert_eq!(serialized["reasoning"]["effort"], "low");
    }

    #[test]
    fn test_transforms_none_serialization() {
        let req = ChatCompletionRequest::new("gpt-4".to_string(), vec![]);
        let serialised = serde_json::to_value(&req).unwrap();
        // Verify that the transforms field is completely omitted from JSON output
        assert!(!serialised.as_object().unwrap().contains_key("transforms"));
    }

    #[test]
    fn test_transforms_some_serialization() {
        let mut req = ChatCompletionRequest::new("gpt-4".to_string(), vec![]);
        req.transforms = Some(vec!["transform1".to_string(), "transform2".to_string()]);
        let serialised = serde_json::to_value(&req).unwrap();
        // Verify that the transforms field is included as a proper JSON array
        assert_eq!(
            serialised["transforms"],
            serde_json::json!(["transform1", "transform2"])
        );
    }

    #[test]
    fn test_transforms_some_deserialization() {
        let json_str =
            r#"{"model": "gpt-4", "messages": [], "transforms": ["transform1", "transform2"]}"#;
        let req: ChatCompletionRequest = serde_json::from_str(json_str).unwrap();
        // Verify that the transforms field is properly populated with Some(vec)
        assert_eq!(
            req.transforms,
            Some(vec!["transform1".to_string(), "transform2".to_string()])
        );
    }

    #[test]
    fn test_transforms_none_deserialization() {
        let json_str = r#"{"model": "gpt-4", "messages": []}"#;
        let req: ChatCompletionRequest = serde_json::from_str(json_str).unwrap();
        // Verify that the transforms field is properly set to None when absent
        assert_eq!(req.transforms, None);
    }

    #[test]
    fn test_transforms_builder_method() {
        let transforms = vec!["transform1".to_string(), "transform2".to_string()];
        let req =
            ChatCompletionRequest::new("gpt-4".to_string(), vec![]).transforms(transforms.clone());
        // Verify that the transforms field is properly set through the builder method
        assert_eq!(req.transforms, Some(transforms));
    }
}
