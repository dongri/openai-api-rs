use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::audio::{self, AudioSpeechRequest, TTS_1};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let req = AudioSpeechRequest::new(
        TTS_1.to_string(),
        String::from("Money is not the problem, the problem is no money."),
        audio::VOICE_ALLOY.to_string(),
        String::from("examples/data/problem.mp3"),
    );

    let result = client.audio_speech(req).await?;
    println!("{:?}", result);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example audio_speech
