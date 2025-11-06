use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::common::GPT4_1_MINI;
use openai_api_rs::v1::responses::CreateResponseRequest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let mut req = CreateResponseRequest::new();
    req.model = Some(GPT4_1_MINI.to_string());
    req.input = Some(json!(
        "Tell me a three sentence bedtime story about a unicorn."
    ));
    req.extra.insert("temperature".to_string(), json!(0.7));

    let resp = client.create_response(req).await?;
    println!("response id: {} status: {:?}", resp.id, resp.status);
    println!("response output: {:?}", resp.output);
    Ok(())
}
