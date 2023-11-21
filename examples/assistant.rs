use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::assistant::AssistantRequest;
use openai_api_rs::v1::common::GPT4_1106_PREVIEW;
use openai_api_rs::v1::message::{CreateMessageRequest, MessageRole};
use openai_api_rs::v1::run::CreateRunRequest;
use openai_api_rs::v1::thread::CreateThreadRequest;
use std::collections::HashMap;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let mut tools = HashMap::new();
    tools.insert("type".to_string(), "code_interpreter".to_string());

    let req = AssistantRequest::new(GPT4_1106_PREVIEW.to_string());
    let req = req
        .clone()
        .description("this is a test assistant".to_string());
    let req = req.clone().instructions("You are a personal math tutor. When asked a question, write and run Python code to answer the question.".to_string());
    let req = req.clone().tools(vec![tools]);
    println!("{:?}", req);

    let result = client.create_assistant(req)?;
    println!("{:?}", result.id);

    let thread_req = CreateThreadRequest::new();
    let thread_result = client.create_thread(thread_req)?;
    println!("{:?}", thread_result.id.clone());

    let message_req = CreateMessageRequest::new(
        MessageRole::user,
        "`I need to solve the equation 3x + 11 = 14. Can you help me?".to_string(),
    );

    let message_result = client.create_message(thread_result.id.clone(), message_req)?;
    println!("{:?}", message_result.id.clone());

    let run_req = CreateRunRequest::new(result.id);
    let run_result = client.create_run(thread_result.id.clone(), run_req)?;

    loop {
        let run_result = client
            .retrieve_run(thread_result.id.clone(), run_result.id.clone())
            .unwrap();
        if run_result.status == "completed" {
            break;
        } else {
            println!("waiting...");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    let list_message_result = client.list_messages(thread_result.id.clone()).unwrap();
    for data in list_message_result.data {
        for content in data.content {
            println!(
                "{:?}: {:?} {:?}",
                data.role, content.text.value, content.text.annotations
            );
        }
    }

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example assistant
