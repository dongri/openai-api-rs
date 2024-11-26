use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::completion::{self, CompletionRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let req = CompletionRequest::new(
        completion::GPT3_TEXT_DAVINCI_003.to_string(),
        String::from("What is Bitcoin?"),
    )
    .max_tokens(3000)
    .temperature(0.9)
    .top_p(1.0)
    .stop(vec![String::from(" Human:"), String::from(" AI:")])
    .presence_penalty(0.6)
    .frequency_penalty(0.0);

    let result = client.completion(req).await?;
    println!("{:}", result.choices[0].text);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example completion
