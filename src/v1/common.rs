use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
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

// https://platform.openai.com/docs/models/gpt-4o-mini
pub const GPT4_O_MINI: &str = "gpt-4o-mini";

// https://platform.openai.com/docs/models/gpt-4o
pub const GPT4_O: &str = "gpt-4o";
pub const GPT4_O_2024_05_13: &str = "gpt-4o-2024-05-13";

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
