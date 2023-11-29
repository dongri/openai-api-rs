# OpenAI API client library for Rust (unofficial)
The OpenAI API client Rust library provides convenient access to the OpenAI API from Rust applications.

Check out the [docs.rs](https://docs.rs/openai-api-rs/).

## Installation:
Cargo.toml
```toml
[dependencies]
openai-api-rs = "2.1.4"
```

## Usage
The library needs to be configured with your account's secret key, which is available on the [website](https://platform.openai.com/account/api-keys). We recommend setting it as an environment variable. Here's an example of initializing the library with the API key loaded from an environment variable and creating a completion:

### Set OPENAI_API_KEY to environment variable
```bash
$ export OPENAI_API_KEY=sk-xxxxxxx
```

### Set OPENAI_API_BASE to environment variable (optional)
```bash
$ export OPENAI_API_BASE=https://api.openai.com/v1
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
use openai_api_rs::v1::common::GPT4;
let req = ChatCompletionRequest::new(
    GPT4.to_string(),
    vec![chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: String::from("Hello OpenAI!"),
    }],
);
```

### Send request
```rust
let result = client.completion(req)?;
println!("{:?}", result.choices[0].text);
```

## Example of chat completion
```rust
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: String::from("What is Bitcoin?"),
            name: None,
            function_call: None,
        }],
    );
    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.content);
    Ok(())
}
```
More Examples: [examples](https://github.com/dongri/openai-api-rs/tree/main/examples)

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
- [x] [Function calling](https://platform.openai.com/docs/guides/gpt/function-calling)
- [x] [Assistants](https://platform.openai.com/docs/assistants/overview)

## License
This project is licensed under [MIT license](https://github.com/dongri/openai-api-rs/blob/main/LICENSE).
