use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;
use openai_api_rs::v1::types;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, vec};

fn get_coin_price(coin: &str) -> f64 {
    let coin = coin.to_lowercase();
    match coin.as_str() {
        "btc" | "bitcoin" => 10000.0,
        "eth" | "ethereum" => 1000.0,
        _ => 0.0,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let mut properties = HashMap::new();
    properties.insert(
        "coin".to_string(),
        Box::new(types::JSONSchemaDefine {
            schema_type: Some(types::JSONSchemaType::String),
            description: Some("The cryptocurrency to get the price of".to_string()),
            ..Default::default()
        }),
    );

    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("What is the price of Ethereum?")),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    )
    .tools(vec![chat_completion::Tool {
        r#type: chat_completion::ToolType::Function,
        function: types::Function {
            name: String::from("get_coin_price"),
            description: Some(String::from("Get the price of a cryptocurrency")),
            parameters: types::FunctionParameters {
                schema_type: types::JSONSchemaType::Object,
                properties: Some(properties),
                required: Some(vec![String::from("coin")]),
            },
        },
    }]);

    let result = client.chat_completion(req).await?;

    match result.choices[0].finish_reason {
        None => {
            println!("No finish_reason");
            println!("{:?}", result.choices[0].message.content);
        }
        Some(chat_completion::FinishReason::stop) => {
            println!("Stop");
            println!("{:?}", result.choices[0].message.content);
        }
        Some(chat_completion::FinishReason::length) => {
            println!("Length");
        }
        Some(chat_completion::FinishReason::tool_calls) => {
            println!("ToolCalls");
            #[derive(Deserialize, Serialize)]
            struct Currency {
                coin: String,
            }
            let tool_calls = result.choices[0].message.tool_calls.as_ref().unwrap();
            for tool_call in tool_calls {
                let function_call = &tool_call.function;
                let arguments = function_call.arguments.clone().unwrap();
                let c: Currency = serde_json::from_str(&arguments)?;
                let coin = c.coin;
                println!("coin: {}", coin);
                let price = get_coin_price(&coin);
                println!("price: {}", price);

                let req = ChatCompletionRequest::new(
                    GPT4_O.to_string(),
                    vec![
                        chat_completion::ChatCompletionMessage {
                            role: chat_completion::MessageRole::user,
                            content: chat_completion::Content::Text(String::from(
                                "What is the price of Ethereum?",
                            )),
                            name: None,
                            tool_calls: None,
                            tool_call_id: None,
                        },
                        chat_completion::ChatCompletionMessage {
                            role: chat_completion::MessageRole::function,
                            content: chat_completion::Content::Text({
                                let price = get_coin_price(&coin);
                                format!("{{\"price\": {}}}", price)
                            }),
                            name: Some(String::from("get_coin_price")),
                            tool_calls: None,
                            tool_call_id: None,
                        },
                    ],
                );

                let result = client.chat_completion(req).await?;
                println!("{:?}", result.choices[0].message.content);
            }
        }
        Some(chat_completion::FinishReason::content_filter) => {
            println!("ContentFilter");
        }
        Some(chat_completion::FinishReason::null) => {
            println!("Null");
        }
    }
    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example function_call_role
