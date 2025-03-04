use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::audio::{AudioTranscriptionRequest, WHISPER_1};
use std::env;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let file_path = "examples/data/problem.mp3";

    // Test with file
    let req = AudioTranscriptionRequest::new(file_path.to_string(), WHISPER_1.to_string());

    let req_json = req.clone().response_format("json".to_string());

    let result = client.audio_transcription(req_json).await?;
    println!("{:?}", result);

    let req_raw = req.clone().response_format("text".to_string());

    let result = client.audio_transcription_raw(req_raw).await?;
    println!("{:?}", result);

    // Test with bytes
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let req = AudioTranscriptionRequest::new_bytes(buffer, WHISPER_1.to_string());

    let req_json = req.clone().response_format("json".to_string());

    let result = client.audio_transcription(req_json).await?;
    println!("{:?}", result);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example audio_transcriptions
