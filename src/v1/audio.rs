use serde::{Deserialize, Serialize};

pub const WHISPER_1: &str = "whisper-1";

#[derive(Debug, Serialize)]
pub struct AudioTranscriptionRequest {
    pub file: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AudioTranscriptionResponse {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AudioTranslationRequest {
    pub file: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct AudioTranslationResponse {
    pub text: String,
}
