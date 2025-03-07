use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::batch::CreateBatchRequest;
use openai_api_rs::v1::file::FileUploadRequest;
use serde_json::{from_str, to_string_pretty, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let req = FileUploadRequest::new(
        "examples/data/batch_request.json".to_string(),
        "batch".to_string(),
    );

    let result = client.upload_file(req).await?;
    println!("File id: {:?}", result.id);

    let input_file_id = result.id;
    let req = CreateBatchRequest::new(
        input_file_id.clone(),
        "/v1/chat/completions".to_string(),
        "24h".to_string(),
    );

    let result = client.create_batch(req).await?;
    println!("Batch id: {:?}", result.id);

    let batch_id = result.id;
    let result = client.retrieve_batch(batch_id.to_string()).await?;
    println!("Batch status: {:?}", result.status);

    // sleep 30 seconds
    println!("Sleeping for 30 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    let result = client.retrieve_batch(batch_id.to_string()).await?;

    let file_id = result.output_file_id.unwrap();
    let result = client.retrieve_file_content(file_id).await?;
    let s = match str::from_utf8(&result) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let json_value: Value = from_str(&s)?;
    let result_json = to_string_pretty(&json_value)?;

    let output_file_path = "examples/data/batch_result.json";
    let mut file = File::create(output_file_path)?;
    file.write_all(result_json.as_bytes())?;

    println!("File writed to {:?}", output_file_path);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example batch
