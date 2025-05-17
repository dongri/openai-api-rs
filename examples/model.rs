use openai_api_rs::v1::api::OpenAIClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let result = client.list_models().await?;
    let models = result.data;

    for model in models {
        println!("Model id: {:?}", model.id);
    }

    let result = client.retrieve_model("gpt-4.1".to_string()).await?;
    println!("Model id: {:?}", result.id);
    println!("Model object: {:?}", result.object);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example model
