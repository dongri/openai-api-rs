use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::embedding::EmbeddingRequest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = EmbeddingRequest {
        model: "text-embedding-ada-002".to_string(),
        input: "story time".to_string(),
        user: Option::None,
    };
    let result = client.embedding(req).await?;
    println!("{:?}", result.data);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example embedding
