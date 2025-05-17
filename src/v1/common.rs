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

// https://platform.openai.com/docs/models/o3
pub const O3: &str = "o3";
pub const O3_2025_04_16: &str = "o3-2025-04-16";
pub const O3_MINI: &str = "o3-mini";
pub const O3_MINI_2025_01_31: &str = "o3-mini-2025-01-31";

// https://platform.openai.com/docs/models#gpt-4-5
pub const GPT4_5_PREVIEW: &str = "gpt-4.5-preview";
pub const GPT4_5_PREVIEW_2025_02_27: &str = "gpt-4.5-preview-2025-02-27";

// https://platform.openai.com/docs/models/o1
pub const O1_PREVIEW: &str = "o1-preview";
pub const O1_PREVIEW_2024_09_12: &str = "o1-preview-2024-09-12";
pub const O1_MINI: &str = "o1-mini";
pub const O1_MINI_2024_09_12: &str = "o1-mini-2024-09-12";

// https://platform.openai.com/docs/models/gpt-4o-mini
pub const GPT4_O_MINI: &str = "gpt-4o-mini";
pub const GPT4_O_MINI_2024_07_18: &str = "gpt-4o-mini-2024-07-18";

// https://platform.openai.com/docs/models/gpt-4o
pub const GPT4_O: &str = "gpt-4o";
pub const GPT4_O_2024_05_13: &str = "gpt-4o-2024-05-13";
pub const GPT4_O_2024_08_06: &str = "gpt-4o-2024-08-06";
pub const GPT4_O_LATEST: &str = "chatgpt-4o-latest";

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
pub const GPT4_0125_PREVIEW: &str = "gpt-4-0125-preview";
pub const GPT4_TURBO_PREVIEW: &str = "gpt-4-turbo-preview";
pub const GPT4_1106_PREVIEW: &str = "gpt-4-1106-preview";
pub const GPT4_VISION_PREVIEW: &str = "gpt-4-vision-preview";
pub const GPT4: &str = "gpt-4";
pub const GPT4_32K: &str = "gpt-4-32k";
pub const GPT4_0613: &str = "gpt-4-0613";
pub const GPT4_32K_0613: &str = "gpt-4-32k-0613";
// - legacy
pub const GPT4_0314: &str = "gpt-4-0314";
pub const GPT4_32K_0314: &str = "gpt-4-32k-0314";

// https://platform.openai.com/docs/api-reference/images/object
pub const DALL_E_2: &str = "dall-e-2";
pub const DALL_E_3: &str = "dall-e-3";

// https://platform.openai.com/docs/guides/embeddings/embedding-models
pub const TEXT_EMBEDDING_3_SMALL: &str = "text-embedding-3-small";
pub const TEXT_EMBEDDING_3_LARGE: &str = "text-embedding-3-large";
pub const TEXT_EMBEDDING_ADA_002: &str = "text-embedding-ada-002";
