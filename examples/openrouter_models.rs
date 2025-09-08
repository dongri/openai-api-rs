use openai_api_rs::v1::api::OpenAIClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENROUTER_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder()
        .with_endpoint("https://openrouter.ai/api/v1")
        .with_api_key(api_key)
        .build()?;

    let result = client.list_models().await?;
    let models = result.data;

    for model in models {
        println!("Model id: {:?}", model.id);
    }

    Ok(())
}

// OPENROUTER_API_KEY=xxxx cargo run --package openai-api-rs --example openrouter_models
