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

// GPT-5.4 models
pub const GPT5_4: &str = "gpt-5.4";
pub const GPT5_4_2026_03_05: &str = "gpt-5.4-2026-03-05";
pub const GPT5_4_MINI: &str = "gpt-5.4-mini";
pub const GPT5_4_MINI_2026_03_17: &str = "gpt-5.4-mini-2026-03-17";
pub const GPT5_4_NANO: &str = "gpt-5.4-nano";
pub const GPT5_4_NANO_2026_03_17: &str = "gpt-5.4-nano-2026-03-17";
pub const GPT5_4_PRO: &str = "gpt-5.4-pro";
pub const GPT5_4_PRO_2026_03_05: &str = "gpt-5.4-pro-2026-03-05";

// GPT-5.3 models
pub const GPT5_3_CHAT_LATEST: &str = "gpt-5.3-chat-latest";
pub const GPT5_3_CODEX: &str = "gpt-5.3-codex";

// GPT-5.2 models
pub const GPT5_2: &str = "gpt-5.2";
pub const GPT5_2_2025_12_11: &str = "gpt-5.2-2025-12-11";
pub const GPT5_2_CHAT_LATEST: &str = "gpt-5.2-chat-latest";
pub const GPT5_2_CODEX: &str = "gpt-5.2-codex";
pub const GPT5_2_PRO: &str = "gpt-5.2-pro";
pub const GPT5_2_PRO_2025_12_11: &str = "gpt-5.2-pro-2025-12-11";

// GPT-5.1 models
pub const GPT5_1: &str = "gpt-5.1";
pub const GPT5_1_2025_11_13: &str = "gpt-5.1-2025-11-13";
pub const GPT5_1_CHAT_LATEST: &str = "gpt-5.1-chat-latest";
pub const GPT5_1_CODEX: &str = "gpt-5.1-codex";
pub const GPT5_1_CODEX_MAX: &str = "gpt-5.1-codex-max";
pub const GPT5_1_CODEX_MINI: &str = "gpt-5.1-codex-mini";

// GPT-5 models
pub const GPT5: &str = "gpt-5";
pub const GPT5_2025_08_07: &str = "gpt-5-2025-08-07";
pub const GPT5_CHAT_LATEST: &str = "gpt-5-chat-latest";
pub const GPT5_CODEX: &str = "gpt-5-codex";
pub const GPT5_MINI: &str = "gpt-5-mini";
pub const GPT5_MINI_2025_08_07: &str = "gpt-5-mini-2025-08-07";
pub const GPT5_NANO: &str = "gpt-5-nano";
pub const GPT5_NANO_2025_08_07: &str = "gpt-5-nano-2025-08-07";
pub const GPT5_PRO: &str = "gpt-5-pro";
pub const GPT5_PRO_2025_10_06: &str = "gpt-5-pro-2025-10-06";
pub const GPT5_SEARCH_API: &str = "gpt-5-search-api";
pub const GPT5_SEARCH_API_2025_10_14: &str = "gpt-5-search-api-2025-10-14";

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

pub const GPT4_O_MINI: &str = "gpt-4o-mini";
pub const GPT4_O_MINI_2024_07_18: &str = "gpt-4o-mini-2024-07-18";

// GPT-4o search models
pub const GPT4_O_SEARCH_PREVIEW: &str = "gpt-4o-search-preview";
pub const GPT4_O_SEARCH_PREVIEW_2025_03_11: &str = "gpt-4o-search-preview-2025-03-11";
pub const GPT4_O_MINI_SEARCH_PREVIEW: &str = "gpt-4o-mini-search-preview";
pub const GPT4_O_MINI_SEARCH_PREVIEW_2025_03_11: &str = "gpt-4o-mini-search-preview-2025-03-11";

// GPT-4o realtime models
pub const GPT4_O_REALTIME_PREVIEW: &str = "gpt-4o-realtime-preview";
pub const GPT4_O_REALTIME_PREVIEW_2024_12_17: &str = "gpt-4o-realtime-preview-2024-12-17";
pub const GPT4_O_REALTIME_PREVIEW_2025_06_03: &str = "gpt-4o-realtime-preview-2025-06-03";
pub const GPT4_O_MINI_REALTIME_PREVIEW: &str = "gpt-4o-mini-realtime-preview";
pub const GPT4_O_MINI_REALTIME_PREVIEW_2024_12_17: &str = "gpt-4o-mini-realtime-preview-2024-12-17";

// GPT-4o audio models
pub const GPT4_O_AUDIO_PREVIEW: &str = "gpt-4o-audio-preview";
pub const GPT4_O_AUDIO_PREVIEW_2024_12_17: &str = "gpt-4o-audio-preview-2024-12-17";
pub const GPT4_O_AUDIO_PREVIEW_2025_06_03: &str = "gpt-4o-audio-preview-2025-06-03";
pub const GPT4_O_MINI_AUDIO_PREVIEW: &str = "gpt-4o-mini-audio-preview";
pub const GPT4_O_MINI_AUDIO_PREVIEW_2024_12_17: &str = "gpt-4o-mini-audio-preview-2024-12-17";

// GPT-4o transcription and TTS models
pub const GPT4_O_TRANSCRIBE: &str = "gpt-4o-transcribe";
pub const GPT4_O_TRANSCRIBE_DIARIZE: &str = "gpt-4o-transcribe-diarize";
pub const GPT4_O_MINI_TRANSCRIBE: &str = "gpt-4o-mini-transcribe";
pub const GPT4_O_MINI_TRANSCRIBE_2025_03_20: &str = "gpt-4o-mini-transcribe-2025-03-20";
pub const GPT4_O_MINI_TRANSCRIBE_2025_12_15: &str = "gpt-4o-mini-transcribe-2025-12-15";
pub const GPT4_O_MINI_TTS: &str = "gpt-4o-mini-tts";
pub const GPT4_O_MINI_TTS_2025_03_20: &str = "gpt-4o-mini-tts-2025-03-20";
pub const GPT4_O_MINI_TTS_2025_12_15: &str = "gpt-4o-mini-tts-2025-12-15";

// GPT-4 models
pub const GPT4: &str = "gpt-4";
pub const GPT4_0613: &str = "gpt-4-0613";
pub const GPT4_TURBO: &str = "gpt-4-turbo";
pub const GPT4_TURBO_2024_04_09: &str = "gpt-4-turbo-2024-04-09";

// GPT-3.5 models
pub const GPT3_5_TURBO: &str = "gpt-3.5-turbo";
pub const GPT3_5_TURBO_0125: &str = "gpt-3.5-turbo-0125";
pub const GPT3_5_TURBO_1106: &str = "gpt-3.5-turbo-1106";
pub const GPT3_5_TURBO_16K: &str = "gpt-3.5-turbo-16k";
pub const GPT3_5_TURBO_INSTRUCT: &str = "gpt-3.5-turbo-instruct";
pub const GPT3_5_TURBO_INSTRUCT_0914: &str = "gpt-3.5-turbo-instruct-0914";

// Audio models
pub const GPT_AUDIO: &str = "gpt-audio";
pub const GPT_AUDIO_1_5: &str = "gpt-audio-1.5";
pub const GPT_AUDIO_2025_08_28: &str = "gpt-audio-2025-08-28";
pub const GPT_AUDIO_MINI: &str = "gpt-audio-mini";
pub const GPT_AUDIO_MINI_2025_10_06: &str = "gpt-audio-mini-2025-10-06";
pub const GPT_AUDIO_MINI_2025_12_15: &str = "gpt-audio-mini-2025-12-15";

pub const GPT_REALTIME: &str = "gpt-realtime";
pub const GPT_REALTIME_1_5: &str = "gpt-realtime-1.5";
pub const GPT_REALTIME_2025_08_28: &str = "gpt-realtime-2025-08-28";
pub const GPT_REALTIME_MINI: &str = "gpt-realtime-mini";
pub const GPT_REALTIME_MINI_2025_10_06: &str = "gpt-realtime-mini-2025-10-06";
pub const GPT_REALTIME_MINI_2025_12_15: &str = "gpt-realtime-mini-2025-12-15";

// Text-to-Speech models
pub const TTS_1: &str = "tts-1";
pub const TTS_1_1106: &str = "tts-1-1106";
pub const TTS_1_HD: &str = "tts-1-hd";
pub const TTS_1_HD_1106: &str = "tts-1-hd-1106";

// Speech-to-Text models
pub const WHISPER_1: &str = "whisper-1";

// Image generation models
pub const CHATGPT_IMAGE_LATEST: &str = "chatgpt-image-latest";
pub const DALL_E_2: &str = "dall-e-2";
pub const DALL_E_3: &str = "dall-e-3";
pub const GPT_IMAGE_1: &str = "gpt-image-1";
pub const GPT_IMAGE_1_5: &str = "gpt-image-1.5";
pub const GPT_IMAGE_1_MINI: &str = "gpt-image-1-mini";

// Embedding models
pub const TEXT_EMBEDDING_3_SMALL: &str = "text-embedding-3-small";
pub const TEXT_EMBEDDING_3_LARGE: &str = "text-embedding-3-large";
pub const TEXT_EMBEDDING_ADA_002: &str = "text-embedding-ada-002";

// Moderation models
pub const OMNI_MODERATION_LATEST: &str = "omni-moderation-latest";
pub const OMNI_MODERATION_2024_09_26: &str = "omni-moderation-2024-09-26";

// Legacy models
pub const BABBAGE_002: &str = "babbage-002";
pub const DAVINCI_002: &str = "davinci-002";

// Video models
pub const SORA_2: &str = "sora-2";
pub const SORA_2_PRO: &str = "sora-2-pro";
