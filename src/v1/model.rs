use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelsResponse {
    pub object: Option<String>,
    pub data: Vec<ModelResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelResponse {
    pub id: Option<String>,
    pub name: Option<String>,
    pub created: Option<i64>,
    pub description: Option<String>,
    pub architecture: Option<Architecture>,
    pub top_provider: Option<TopProvider>,
    pub pricing: Option<Pricing>,
    pub canonical_slug: Option<String>,
    pub context_length: Option<i64>,
    pub hugging_face_id: Option<String>,
    pub per_request_limits: Option<serde_json::Value>,
    pub supported_parameters: Option<Vec<String>>,
    pub object: Option<String>,
    pub owned_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Architecture {
    pub input_modalities: Option<Vec<String>>,
    pub output_modalities: Option<Vec<String>>,
    pub tokenizer: Option<String>,
    pub instruct_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TopProvider {
    pub is_moderated: Option<bool>,
    pub context_length: Option<i64>,
    pub max_completion_tokens: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pricing {
    pub prompt: Option<String>,
    pub completion: Option<String>,
    pub image: Option<String>,
    pub request: Option<String>,
    pub web_search: Option<String>,
    pub internal_reasoning: Option<String>,
    pub input_cache_read: Option<String>,
    pub input_cache_write: Option<String>,
}
