use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::impl_builder_methods;

pub const WHISPER_1: &str = "whisper-1";

#[derive(Debug, Serialize, Clone)]
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

impl AudioTranscriptionRequest {
    pub fn new(file: String, model: String) -> Self {
        Self {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None,
        }
    }
}

impl_builder_methods!(
    AudioTranscriptionRequest,
    prompt: String,
    response_format: String,
    temperature: f32,
    language: String
);

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioTranscriptionResponse {
    pub text: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Clone)]
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

impl AudioTranslationRequest {
    pub fn new(file: String, model: String) -> Self {
        Self {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
        }
    }
}

impl_builder_methods!(
    AudioTranslationRequest,
    prompt: String,
    response_format: String,
    temperature: f32
);

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioTranslationResponse {
    pub text: String,
    pub headers: Option<HashMap<String, String>>,
}

pub const TTS_1: &str = "tts-1";
pub const TTS_1_HD: &str = "tts-1-hd";

pub const VOICE_ALLOY: &str = "alloy";
pub const VOICE_ECHO: &str = "echo";
pub const VOICE_FABLE: &str = "fable";
pub const VOICE_ONYX: &str = "onyx";
pub const VOICE_NOVA: &str = "nova";
pub const VOICE_SHIMMER: &str = "shimmer";

#[derive(Debug, Serialize, Clone)]
pub struct AudioSpeechRequest {
    pub model: String,
    pub input: String,
    pub voice: String,
    pub output: String,
}

impl AudioSpeechRequest {
    pub fn new(model: String, input: String, voice: String, output: String) -> Self {
        Self {
            model,
            input,
            voice,
            output,
        }
    }
}

impl_builder_methods!(AudioSpeechRequest,);

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioSpeechResponse {
    pub result: bool,
    pub headers: Option<HashMap<String, String>>,
}
