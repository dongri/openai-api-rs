use llm_budget::AgentBudget;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::chat_completion::{self};
use openai_api_rs::v1::common::GPT4_O_MINI;
use std::env;

// Ask gpt-4o-mini a series of questions against a $0.0001 hard cap: each
// answer is charged at the model's real price, and once the cap is spent the
// remaining questions are refused before any API call goes out.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = OpenAIClient::builder().with_api_key(api_key).build()?;
    let budget = AgentBudget::builder("chat", 0.0001).build()?;
    let model = GPT4_O_MINI;

    for question in [
        "What is the capital of France?",
        "And of Japan?",
        "And of Brazil?",
        "And of Canada?",
        "And of Australia?",
    ] {
        if budget.is_exhausted() {
            println!(
                "{question} -> skipped: ${:.4} of ${:.4} spent",
                budget.spent_usd(),
                budget.limit_usd()
            );
            continue;
        }

        let req = ChatCompletionRequest::new(
            model.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(String::from("Answer in one word.")),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }, chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(String::from(question)),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        );

        let result = client.chat_completion(req).await?;
        let usage = result.inner.usage;
        let receipt = budget
            .charge_model(
                model,
                usage.prompt_tokens as u32,
                usage.completion_tokens as u32,
                None,
                None,
            )
            .await?;
        let answer = result.inner.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default();
        println!(
            "{question} -> {answer} (${:.6} this call, ${:.6} total)",
            receipt.cost_usd, receipt.total_spent_usd
        );
    }

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example cost_cap
