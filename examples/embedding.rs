use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::embedding::EmbeddingRequest;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = EmbeddingRequest::new(
        "text-embedding-ada-002".to_string(),
        "story time".to_string(),
    );

    let result = client.embedding(req)?;
    println!("{:?}", result.data);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example embedding
