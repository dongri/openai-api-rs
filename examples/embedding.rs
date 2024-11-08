use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::common::TEXT_EMBEDDING_3_SMALL;
use openai_api_rs::v1::embedding::EmbeddingRequest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let mut req = EmbeddingRequest::new(
        TEXT_EMBEDDING_3_SMALL.to_string(),
        vec!["story time".to_string(), "Once upon a time".to_string()],
    );
    req.dimensions = Some(10);

    let result = client.embedding(req).await?;
    println!("{:?}", result.data);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example embedding
