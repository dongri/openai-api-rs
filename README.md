# OpenAI API client library for Rust (unofficial)

The OpenAI API client Rust library provides convenient access to the OpenAI API from Rust applications.

Check out the [docs.rs](https://docs.rs/openai-api-rs/).

## Installation:

Cargo.toml

```toml
[dependencies]
openai-api-rs = "8.0.0"
```

## Usage

The library needs to be configured with your account's secret key, which is available on the [website](https://platform.openai.com/account/api-keys). We recommend setting it as an environment variable. Here's an example of initializing the library with the API key loaded from an environment variable and creating a completion:

### Set OPENAI_API_KEY or OPENROUTER_API_KEY to environment variable

```bash
$ export OPENAI_API_KEY=sk-xxxxxxx
or
$ export OPENROUTER_API_KEY=sk-xxxxxxx
```

### Create OpenAI client

```rust
let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;
```

### Create OpenRouter client

```rust
let api_key = env::var("OPENROUTER_API_KEY").unwrap().to_string();
let mut client = OpenAIClient::builder()
    .with_endpoint("https://openrouter.ai/api/v1")
    .with_api_key(api_key)
    .build()?;
```

### Create request

```rust
let req = ChatCompletionRequest::new(
    GPT4_O.to_string(),
    vec![chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from("What is bitcoin?")),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }],
);
```

### Send request

```rust
let result = client.chat_completion(req)?;
println!("Content: {:?}", result.choices[0].message.content);

for (key, value) in client.headers.unwrap().iter() {
    println!("{}: {:?}", key, value);
}
```

### Set OPENAI_API_BASE to environment variable (optional)

```bash
$ export OPENAI_API_BASE=https://api.openai.com/v1
```

## Example of chat completion

```rust
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("What is bitcoin?")),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    let result = client.chat_completion(req).await?;
    println!("Content: {:?}", result.choices[0].message.content);

    for (key, value) in client.headers.unwrap().iter() {
        println!("{}: {:?}", key, value);
    }

    Ok(())
}
```

## Example for OpenRouter

```rust
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O_MINI;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENROUTER_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder()
        .with_endpoint("https://openrouter.ai/api/v1")
        .with_api_key(api_key)
        .build()?;

    let req = ChatCompletionRequest::new(
        GPT4_O_MINI.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(String::from("What is bitcoin?")),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    let result = client.chat_completion(req).await?;
    println!("Content: {:?}", result.choices[0].message.content);

    for (key, value) in client.headers.unwrap().iter() {
        println!("{}: {:?}", key, value);
    }

    Ok(())
}
```

More Examples: [examples](https://github.com/dongri/openai-api-rs/tree/main/examples)

Check out the [full API documentation](https://platform.openai.com/docs/api-reference/completions) for examples of all the available functions.

## Supported APIs

- [x] [Completions](https://platform.openai.com/docs/api-reference/completions)
- [x] [Chat](https://platform.openai.com/docs/api-reference/chat)
- [x] [Edits](https://platform.openai.com/docs/api-reference/edits)
- [x] [Images](https://platform.openai.com/docs/api-reference/images)
- [x] [Embeddings](https://platform.openai.com/docs/api-reference/embeddings)
- [x] [Audio](https://platform.openai.com/docs/api-reference/audio)
- [x] [Files](https://platform.openai.com/docs/api-reference/files)
- [x] [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
- [x] [Moderations](https://platform.openai.com/docs/api-reference/moderations)
- [x] [Function calling](https://platform.openai.com/docs/guides/gpt/function-calling)
- [x] [Assistants](https://platform.openai.com/docs/assistants/overview)
- [x] [Batch](https://platform.openai.com/docs/api-reference/batch)
- [x] [Realtime](https://platform.openai.com/docs/api-reference/realtime)

## License

This project is licensed under [MIT license](https://github.com/dongri/openai-api-rs/blob/main/LICENSE).
