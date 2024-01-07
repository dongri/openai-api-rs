use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_VISION_PREVIEW;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = ChatCompletionRequest::new(
        GPT4_VISION_PREVIEW.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::ImageUrl(vec![
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::text,
                    text: Some(String::from("What’s in this image?")),
                    image_url: None,
                },
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::image_url,
                    text: None,
                    image_url: Some(chat_completion::ImageUrlType {
                        url: String::from(
                            "https://upload.wikimedia.org/wikipedia/commons/5/50/Bitcoin.png",
                        ),
                    }),
                },
            ]),
            name: None,
        }],
    );

    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.content);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example vision
