use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::chat_completion::{self};
use openai_api_rs::v1::common::GPT5_4;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let mut req = ChatCompletionRequest::new(
        GPT5_4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("What is bitcoin?")),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    req.reasoning_effort = Some(chat_completion::ReasoningEffort::High);

    let result = client.chat_completion(req).await?;
    println!("Content: {:?}", result.inner.choices[0].message.content);

    // print response headers
    for (key, value) in result.headers.iter() {
        println!("{}: {:?}", key, value);
    }

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example chat_completion
