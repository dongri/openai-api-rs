use futures_util::StreamExt;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::chat_completion_stream::{
    ChatCompletionStreamRequest, ChatCompletionStreamResponse,
};
use openai_api_rs::v1::chat_completion::{self};
use openai_api_rs::v1::common::GPT4_O_MINI;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let req = ChatCompletionStreamRequest::new(
        GPT4_O_MINI.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("What is bitcoin?")),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    let mut result = client.chat_completion_stream(req).await?;
    while let Some(response) = result.next().await {
        match response.clone() {
            ChatCompletionStreamResponse::ToolCall(toolcalls) => {
                println!("Tool Call: {:?}", toolcalls);
            }
            ChatCompletionStreamResponse::Content(content) => {
                println!("Content: {:?}", content);
            }
            ChatCompletionStreamResponse::Done => {
                println!("Done");
            }
        }
    }

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example chat_completion
