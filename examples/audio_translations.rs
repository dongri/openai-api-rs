use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::audio::{AudioTranslationRequest, WHISPER_1};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = AudioTranslationRequest::new(
        "examples/data/problem_cn.mp3".to_string(),
        WHISPER_1.to_string(),
    );

    let result = client.audio_translation(req).await?;
    println!("{:?}", result);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example audio_transcriptions
