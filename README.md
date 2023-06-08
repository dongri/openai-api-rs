# OpenAI API client library for Rust (unofficial)
The OpenAI API client Rust library provides convenient access to the OpenAI API from Rust applications.

Check out the [docs.rs](https://docs.rs/openai-api-rs/0.1.7/openai_api_rs/v1/index.html).

## Installation:
Cargo.toml
```toml
[dependencies]
openai-api-rs = "0.1.7"
```

## Usage
The library needs to be configured with your account's secret key, which is available on the [website](https://platform.openai.com/account/api-keys). We recommend setting it as an environment variable. Here's an example of initializing the library with the API key loaded from an environment variable and creating a completion:

### Set OPENAI_API_KEY to environment variable
```bash
$ export OPENAI_API_KEY=sk-xxxxxxx
```

### Create client
```rust
use openai_api_rs::v1::api::Client;
use std::env;
let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
```

### Create request
```rust
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
let req = ChatCompletionRequest {
    model: chat_completion::GPT4.to_string(),
    messages: vec![chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: String::from("Hello OpenAI!"),
    }],
};
```

### Send request
```rust
let result = client.completion(req).await?;
println!("{:?}", result.choices[0].text);
```

## Example of chat completion
```rust
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = ChatCompletionRequest {
        model: chat_completion::GPT4.to_string(),
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: String::from("Hello OpenAI!"),
        }],
    };
    let result = client.chat_completion(req).await?;
    println!("{:?}", result.choices[0].message.content);
    Ok(())
}
```
Check out the [full API documentation](https://platform.openai.com/docs/api-reference/completions) for examples of all the available functions.

## Supported APIs
- [x] [completions](https://platform.openai.com/docs/api-reference/completions)
- [x] [Chat](https://platform.openai.com/docs/api-reference/chat)
- [x] [Edits](https://platform.openai.com/docs/api-reference/edits)
- [x] [Images](https://platform.openai.com/docs/api-reference/images)
- [x] [Embeddings](https://platform.openai.com/docs/api-reference/embeddings)
- [x] [Audio](https://platform.openai.com/docs/api-reference/audio)
- [x] [Files](https://platform.openai.com/docs/api-reference/files)
- [x] [Fine-tunes](https://platform.openai.com/docs/api-reference/fine-tunes)
- [x] [Moderations](https://platform.openai.com/docs/api-reference/moderations)

## License
This project is licensed under [MIT license](https://github.com/dongri/openai-api-rs/blob/main/LICENSE).
