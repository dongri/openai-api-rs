# OpenAI API client library for Rust

## Installation:
Cargo.toml
```
[dependencies]
openai-rs = { git = "https://github.com/dongri/openai-rs" }
```

## Example:
```bash
export OPENAI_API_KEY={YOUR_API}
```

```rust
use openai_rs::v1::completion::{self, CompletionRequest};
use openai_rs::v1::api::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = CompletionRequest {
        model: completion::GPT3_TEXT_DAVINCI_003.to_string(),
        prompt: Some(String::from("NFTとは？")),
        suffix: None,
        max_tokens: Some(3000),
        temperature: Some(0.9),
        top_p: Some(1.0),
        n: None,
        stream: None,
        logprobs: None,
        echo: None,
        stop: None,
        presence_penalty: Some(0.6),
        frequency_penalty: Some(0.0),
        best_of: None,
        logit_bias: None,
        user: None,
      };
    let completion_response = client.completion(req).await?;
    println!("{:?}", completion_response.choices[0].text);

    Ok(())
}
```
