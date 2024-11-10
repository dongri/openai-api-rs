use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::audio::{AudioTranscriptionRequest, WHISPER_1};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let req = AudioTranscriptionRequest::new(
        "examples/data/problem.mp3".to_string(),
        WHISPER_1.to_string(),
    );

    let result = client.audio_transcription(req).await?;
    println!("{:?}", result);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example audio_translations
