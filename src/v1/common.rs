use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeletionStatus {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[macro_export]
macro_rules! impl_builder_methods {
    ($builder:ident, $($field:ident: $field_type:ty),*) => {
        impl $builder {
            $(
                pub fn $field(mut self, $field: $field_type) -> Self {
                    self.$field = Some($field);
                    self
                }
            )*
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyRequestBody {}

// O-series models
pub const O1: &str = "o1";
pub const O1_2024_12_17: &str = "o1-2024-12-17";
pub const O1_MINI: &str = "o1-mini";
pub const O1_MINI_2024_09_12: &str = "o1-mini-2024-09-12";
pub const O1_PREVIEW: &str = "o1-preview";
pub const O1_PREVIEW_2024_09_12: &str = "o1-preview-2024-09-12";
pub const O1_PRO: &str = "o1-pro";
pub const O1_PRO_2025_03_19: &str = "o1-pro-2025-03-19";

pub const O3: &str = "o3";
pub const O3_2025_04_16: &str = "o3-2025-04-16";
pub const O3_MINI: &str = "o3-mini";
pub const O3_MINI_2025_01_31: &str = "o3-mini-2025-01-31";

pub const O4_MINI: &str = "o4-mini";
pub const O4_MINI_2025_04_16: &str = "o4-mini-2025-04-16";
pub const O4_MINI_DEEP_RESEARCH: &str = "o4-mini-deep-research";
pub const O4_MINI_DEEP_RESEARCH_2025_06_26: &str = "o4-mini-deep-research-2025-06-26";

// GPT-5 models
pub const GPT5: &str = "gpt-5";
pub const GPT5_2025_08_07: &str = "gpt-5-2025-08-07";
pub const GPT5_CHAT_LATEST: &str = "gpt-5-chat-latest";
pub const GPT5_CODEX: &str = "gpt-5-codex";
pub const GPT5_MINI: &str = "gpt-5-mini";
pub const GPT5_MINI_2025_08_07: &str = "gpt-5-mini-2025-08-07";
pub const GPT5_NANO: &str = "gpt-5-nano";
pub const GPT5_NANO_2025_08_07: &str = "gpt-5-nano-2025-08-07";

// GPT-4.1 models
pub const GPT4_1: &str = "gpt-4.1";
pub const GPT4_1_2025_04_14: &str = "gpt-4.1-2025-04-14";
pub const GPT4_1_MINI: &str = "gpt-4.1-mini";
pub const GPT4_1_MINI_2025_04_14: &str = "gpt-4.1-mini-2025-04-14";
pub const GPT4_1_NANO: &str = "gpt-4.1-nano";
pub const GPT4_1_NANO_2025_04_14: &str = "gpt-4.1-nano-2025-04-14";

// GPT-4o models
pub const GPT4_O: &str = "gpt-4o";
pub const GPT4_O_2024_05_13: &str = "gpt-4o-2024-05-13";
pub const GPT4_O_2024_08_06: &str = "gpt-4o-2024-08-06";
pub const GPT4_O_2024_11_20: &str = "gpt-4o-2024-11-20";
pub const GPT4_O_LATEST: &str = "chatgpt-4o-latest";

pub const GPT4_O_MINI: &str = "gpt-4o-mini";
pub const GPT4_O_MINI_2024_07_18: &str = "gpt-4o-mini-2024-07-18";

// GPT-4o search models
pub const GPT4_O_SEARCH_PREVIEW: &str = "gpt-4o-search-preview";
pub const GPT4_O_SEARCH_PREVIEW_2025_03_11: &str = "gpt-4o-search-preview-2025-03-11";
pub const GPT4_O_MINI_SEARCH_PREVIEW: &str = "gpt-4o-mini-search-preview";
pub const GPT4_O_MINI_SEARCH_PREVIEW_2025_03_11: &str = "gpt-4o-mini-search-preview-2025-03-11";

// GPT-4o realtime models
pub const GPT4_O_REALTIME_PREVIEW: &str = "gpt-4o-realtime-preview";
pub const GPT4_O_REALTIME_PREVIEW_2024_10_01: &str = "gpt-4o-realtime-preview-2024-10-01";
pub const GPT4_O_REALTIME_PREVIEW_2024_12_17: &str = "gpt-4o-realtime-preview-2024-12-17";
pub const GPT4_O_REALTIME_PREVIEW_2025_06_03: &str = "gpt-4o-realtime-preview-2025-06-03";
pub const GPT4_O_MINI_REALTIME_PREVIEW: &str = "gpt-4o-mini-realtime-preview";
pub const GPT4_O_MINI_REALTIME_PREVIEW_2024_12_17: &str = "gpt-4o-mini-realtime-preview-2024-12-17";

// GPT-4o audio models
pub const GPT4_O_AUDIO_PREVIEW: &str = "gpt-4o-audio-preview";
pub const GPT4_O_AUDIO_PREVIEW_2024_10_01: &str = "gpt-4o-audio-preview-2024-10-01";
pub const GPT4_O_AUDIO_PREVIEW_2024_12_17: &str = "gpt-4o-audio-preview-2024-12-17";
pub const GPT4_O_AUDIO_PREVIEW_2025_06_03: &str = "gpt-4o-audio-preview-2025-06-03";
pub const GPT4_O_MINI_AUDIO_PREVIEW: &str = "gpt-4o-mini-audio-preview";
pub const GPT4_O_MINI_AUDIO_PREVIEW_2024_12_17: &str = "gpt-4o-mini-audio-preview-2024-12-17";

// GPT-4o transcription models
pub const GPT4_O_TRANSCRIBE: &str = "gpt-4o-transcribe";
pub const GPT4_O_MINI_TRANSCRIBE: &str = "gpt-4o-mini-transcribe";

// GPT-4 and GPT-4 Turbo models
pub const GPT4: &str = "gpt-4";
pub const GPT4_0613: &str = "gpt-4-0613";
pub const GPT4_32K: &str = "gpt-4-32k";
pub const GPT4_32K_0613: &str = "gpt-4-32k-0613";
pub const GPT4_0314: &str = "gpt-4-0314";
pub const GPT4_32K_0314: &str = "gpt-4-32k-0314";

pub const GPT4_TURBO: &str = "gpt-4-turbo";
pub const GPT4_TURBO_2024_04_09: &str = "gpt-4-turbo-2024-04-09";
pub const GPT4_TURBO_PREVIEW: &str = "gpt-4-turbo-preview";
pub const GPT4_0125_PREVIEW: &str = "gpt-4-0125-preview";
pub const GPT4_1106_PREVIEW: &str = "gpt-4-1106-preview";
pub const GPT4_VISION_PREVIEW: &str = "gpt-4-vision-preview";

// GPT-3.5 Turbo models
pub const GPT3_5_TURBO: &str = "gpt-3.5-turbo";
pub const GPT3_5_TURBO_0125: &str = "gpt-3.5-turbo-0125";
pub const GPT3_5_TURBO_1106: &str = "gpt-3.5-turbo-1106";
pub const GPT3_5_TURBO_16K: &str = "gpt-3.5-turbo-16k";
pub const GPT3_5_TURBO_0613: &str = "gpt-3.5-turbo-0613";
pub const GPT3_5_TURBO_16K_0613: &str = "gpt-3.5-turbo-16k-0613";
pub const GPT3_5_TURBO_0301: &str = "gpt-3.5-turbo-0301";

pub const GPT3_5_TURBO_INSTRUCT: &str = "gpt-3.5-turbo-instruct";
pub const GPT3_5_TURBO_INSTRUCT_0914: &str = "gpt-3.5-turbo-instruct-0914";

// Audio models
pub const GPT_AUDIO: &str = "gpt-audio";
pub const GPT_AUDIO_2025_08_28: &str = "gpt-audio-2025-08-28";
pub const GPT_REALTIME: &str = "gpt-realtime";
pub const GPT_REALTIME_2025_08_28: &str = "gpt-realtime-2025-08-28";

// Text-to-Speech models
pub const TTS_1: &str = "tts-1";
pub const TTS_1_HD: &str = "tts-1-hd";
pub const TTS_1_1106: &str = "tts-1-1106";
pub const TTS_1_HD_1106: &str = "tts-1-hd-1106";
pub const GPT4_O_MINI_TTS: &str = "gpt-4o-mini-tts";

// Speech-to-Text models
pub const WHISPER_1: &str = "whisper-1";

// Image generation models
pub const DALL_E_2: &str = "dall-e-2";
pub const DALL_E_3: &str = "dall-e-3";
pub const GPT_IMAGE_1: &str = "gpt-image-1";

// Embedding models
pub const TEXT_EMBEDDING_3_SMALL: &str = "text-embedding-3-small";
pub const TEXT_EMBEDDING_3_LARGE: &str = "text-embedding-3-large";
pub const TEXT_EMBEDDING_ADA_002: &str = "text-embedding-ada-002";

// Moderation models
pub const OMNI_MODERATION_LATEST: &str = "omni-moderation-latest";
pub const OMNI_MODERATION_2024_09_26: &str = "omni-moderation-2024-09-26";

// Legacy models
pub const DAVINCI_002: &str = "davinci-002";
pub const BABBAGE_002: &str = "babbage-002";

// Code models
pub const CODEX_MINI_LATEST: &str = "codex-mini-latest";

// Preview models (GPT-4.5)
pub const GPT4_5_PREVIEW: &str = "gpt-4.5-preview";
pub const GPT4_5_PREVIEW_2025_02_27: &str = "gpt-4.5-preview-2025-02-27";
