use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bytes::Bytes;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp_granularities[]")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
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
            timestamp_granularities: None,
        }
    }
}

impl_builder_methods!(
    AudioTranscriptionRequest,
    prompt: String,
    response_format: String,
    temperature: f32,
    language: String,
    timestamp_granularities: Vec<TimestampGranularity>
);

#[derive(Debug, Serialize, Clone)]
pub struct AudioTranscriptionRawRequest {
    pub file_name: String,
    pub file_content: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp_granularities[]")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}

impl AudioTranscriptionRawRequest {
    pub fn new(file_name: String, file_content: String, model: String) -> Self {
        Self {
            file_name,
            file_content,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None,
            timestamp_granularities: None,
        }
    }
}

impl_builder_methods!(
    AudioTranscriptionRawRequest,
    prompt: String,
    response_format: String,
    temperature: f32,
    language: String,
    timestamp_granularities: Vec<TimestampGranularity>
);

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AudioTranscriptionResponse {
    Verbose {
        task: String,
        language: String,
        duration: f64,
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        words: Option<Vec<TranscriptionWord>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        segments: Option<Vec<TranscriptionSegment>>,
        headers: Option<HashMap<String, String>>,
    },
    Simple {
        text: String,
        headers: Option<HashMap<String, String>>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TranscriptionWord {
    pub word: String,
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TranscriptionSegment {
    pub id: i32,
    pub seek: i32,
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub tokens: Vec<i32>,
    pub temperature: f64,
    pub avg_logprob: f64,
    pub compression_ratio: f64,
    pub no_speech_prob: f64,
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

#[derive(Debug)]
pub struct AudioSpeechResponse {
    pub result: bool,
    pub headers: Option<HeaderMap>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TimestampGranularity {
    Word,
    Segment,
}
