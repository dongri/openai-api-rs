use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("What is bitcoin?")),
            name: None,
            tool_calls: None,
        }],
    );

    let result = client.chat_completion(req).await?;
    println!("Content: {:?}", result.choices[0].message.content);
    println!("Response Headers: {:?}", result.headers);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example chat_completion
