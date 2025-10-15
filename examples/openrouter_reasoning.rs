use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::chat_completion::{self, Reasoning, ReasoningEffort, ReasoningMode};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENROUTER_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder()
        .with_endpoint("https://openrouter.ai/api/v1")
        .with_api_key(api_key)
        .build()?;

    // Example 1: Using reasoning with effort
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

    // Set reasoning with high effort
    req.reasoning = Some(Reasoning {
        mode: Some(ReasoningMode::Effort {
            effort: ReasoningEffort::High,
        }),
        exclude: Some(false), // Include reasoning in response
        enabled: None,
    });

    let result = client.chat_completion(req).await?;
    println!("Content: {:?}", result.choices[0].message.content);

    // Example 2: Using reasoning with max_tokens
    let mut req2 = ChatCompletionRequest::new(
        "anthropic/claude-4-sonnet".to_string(), // Claude model that supports max_tokens
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

    // Set reasoning with max_tokens
    req2.reasoning = Some(Reasoning {
        mode: Some(ReasoningMode::MaxTokens { max_tokens: 2000 }),
        exclude: None,
        enabled: None,
    });

    let result2 = client.chat_completion(req2).await?;
    println!("Content: {:?}", result2.choices[0].message.content);

    Ok(())
}

// OPENROUTER_API_KEY=xxxx cargo run --package openai-api-rs --example openrouter_reasoning
