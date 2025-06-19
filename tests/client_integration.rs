use serde_json::json;
use openai_api_rs::v1::api::OpenAIClient;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn test_completion_returns_expected_text() {
    // Start a mock server
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "cmpl-12345",
            "object": "text_completion",
            "created": 1633072800,
            "model": "text-davinci-003",
            "choices": [
                {
                    "text": "Bitcoin is a decentralized digital currency.",
                    "index": 0,
                    "logprobs": null,
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = OpenAIClient::builder()
        .with_endpoint(mock_server.uri())
        .with_api_key("test_api_key")
        .build()
        .unwrap();

    let response = client
        .completion(
            openai_api_rs::v1::completion::CompletionRequest::new(
                openai_api_rs::v1::completion::GPT3_TEXT_DAVINCI_003.to_string(),
                "What is Bitcoin?".to_string(),
            )
                .max_tokens(100),
        )
        .await;

    assert!(response.is_ok());

    let result = response.unwrap();
    assert_eq!(result.id, "cmpl-12345");
    assert_eq!(result.model, "text-davinci-003");
    assert_eq!(
        result.choices[0].text.trim(),
        "Bitcoin is a decentralized digital currency."
    );
}
