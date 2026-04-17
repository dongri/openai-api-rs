use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::chat_completion::{self, Reasoning, ReasoningEffort, ReasoningSummary};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENROUTER_API_KEY").unwrap().to_string();
    let client = OpenAIClient::builder()
        .with_endpoint("https://openrouter.ai/api/v1")
        .with_api_key(api_key)
        .build()?;

    // Example 1: OpenRouter reasoning uses the reasoning object.
    let mut req = ChatCompletionRequest::new(
        "x-ai/grok-3-mini".to_string(), // Grok model that supports reasoning
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(
                "Explain quantum computing in simple terms.",
            )),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    req.reasoning = Some(Reasoning {
        effort: Some(ReasoningEffort::High),
        summary: Some(ReasoningSummary::Detailed),
    });

    let result = client.chat_completion(req).await?;
    println!("Content: {:?}", result.inner.choices[0].message.content);
    println!(
        "Reasoning: {:?}",
        result.inner.choices[0].message.reasoning_content
    );

    // Example 2: Another reasoning configuration with a different summary verbosity.
    let mut req2 = ChatCompletionRequest::new(
        "anthropic/claude-opus-4.6".to_string(), // Claude model that supports max_tokens
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from(
                "What's the most efficient sorting algorithm?",
            )),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    req2.reasoning = Some(Reasoning {
        effort: Some(ReasoningEffort::Minimal),
        summary: Some(ReasoningSummary::Concise),
    });

    let result2 = client.chat_completion(req2).await?;
    println!("Content: {:?}", result2.inner.choices[0].message.content);
    println!(
        "Reasoning: {:?}",
        result2.inner.choices[0].message.reasoning_content
    );

    Ok(())
}

// OPENROUTER_API_KEY=xxxx cargo run --package openai-api-rs --example openrouter_reasoning
