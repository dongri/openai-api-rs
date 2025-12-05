use futures_util::StreamExt;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::common::GPT4_1_MINI;
use openai_api_rs::v1::responses::responses_stream::{
    CreateResponseStreamRequest, ResponseStreamResponse,
};
use serde_json::{json, Value};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let mut req = CreateResponseStreamRequest::new();
    req.model = Some(GPT4_1_MINI.to_string());
    req.input = Some(json!("What is bitcoin? Please answer in detail."));

    let mut stream = client.create_response_stream(req).await?;
    let mut full_text = String::new();

    while let Some(event) = stream.next().await {
        match event {
            ResponseStreamResponse::Event(evt) => {
                if let Some("response.output_text.delta") = evt.event.as_deref() {
                    if let Some(delta) = evt.data.get("delta").and_then(Value::as_str) {
                        print!("{delta}");
                        io::stdout().flush()?;
                        full_text.push_str(delta);
                        continue;
                    }
                }

                if let Some(name) = evt.event.as_deref() {
                    println!("\nEvent: {name} => {}", evt.data);
                } else {
                    println!("Event data: {}", evt.data);
                }
            }
            ResponseStreamResponse::Done => {
                println!("\n\nDone streaming response.");
            }
        }
    }

    println!("\nCollected text: {full_text}");
    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example responses_stream
