use crate::v1::assistant::{
    AssistantFileObject, AssistantFileRequest, AssistantObject, AssistantRequest, ListAssistant,
    ListAssistantFile,
};
use crate::v1::audio::{
    AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest, AudioTranscriptionResponse,
    AudioTranslationRequest, AudioTranslationResponse,
};
use crate::v1::batch::{BatchResponse, CreateBatchRequest, ListBatchResponse};
use crate::v1::chat_completion::chat_completion::{ChatCompletionRequest, ChatCompletionResponse};
use crate::v1::chat_completion::chat_completion_stream::{
    ChatCompletionStream, ChatCompletionStreamRequest, ChatCompletionStreamResponse,
};
use crate::v1::common;
use crate::v1::completion::{CompletionRequest, CompletionResponse};
use crate::v1::edit::{EditRequest, EditResponse};
use crate::v1::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::v1::error::APIError;
use crate::v1::file::{
    FileDeleteRequest, FileDeleteResponse, FileListResponse, FileRetrieveResponse,
    FileUploadRequest, FileUploadResponse,
};
use crate::v1::fine_tuning::{
    CancelFineTuningJobRequest, CreateFineTuningJobRequest, FineTuningJobEvent,
    FineTuningJobObject, FineTuningPagination, ListFineTuningJobEventsRequest,
    RetrieveFineTuningJobRequest,
};
use crate::v1::image::{
    ImageEditRequest, ImageEditResponse, ImageGenerationRequest, ImageGenerationResponse,
    ImageVariationRequest, ImageVariationResponse,
};
use crate::v1::message::{
    CreateMessageRequest, ListMessage, ListMessageFile, MessageFileObject, MessageObject,
    ModifyMessageRequest,
};
use crate::v1::model::{ModelResponse, ModelsResponse};
use crate::v1::moderation::{CreateModerationRequest, CreateModerationResponse};
use crate::v1::responses::responses::{
    CallResponse, CountTokensRequest, CountTokensResponse, CreateResponseRequest, ListResponses,
    ResponseObject,
};
use crate::v1::responses::responses_stream::{
    CreateResponseStreamRequest, ResponseStream, ResponseStreamResponse,
};
use crate::v1::run::{
    CreateRunRequest, CreateThreadAndRunRequest, ListRun, ListRunStep, ModifyRunRequest, RunObject,
    RunStepObject,
};
use crate::v1::thread::{CreateThreadRequest, ModifyThreadRequest, ThreadObject};

use bytes::Bytes;
use futures_util::Stream;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Method, Response};
use serde::Serialize;
use serde_json::{to_value, Value};
use url::Url;

use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::io::Write;
use std::path::Path;

const API_URL_V1: &str = "https://api.openai.com/v1";

#[derive(Default)]
pub struct OpenAIClientBuilder {
    api_endpoint: Option<String>,
    api_key: Option<String>,
    organization: Option<String>,
    proxy: Option<String>,
    timeout: Option<u64>,
    headers: Option<HeaderMap>,
}

#[derive(Debug)]
pub struct OpenAIClient {
    api_endpoint: String,
    api_key: Option<String>,
    organization: Option<String>,
    proxy: Option<String>,
    timeout: Option<u64>,
    headers: Option<HeaderMap>,
    pub response_headers: Option<HeaderMap>,
}

impl OpenAIClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.api_endpoint = Some(endpoint.into());
        self
    }

    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    pub fn with_proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let headers = self.headers.get_or_insert_with(HeaderMap::new);
        headers.insert(
            HeaderName::from_bytes(key.into().as_bytes()).expect("Invalid header name"),
            HeaderValue::from_str(&value.into()).expect("Invalid header value"),
        );
        self
    }

    pub fn build(self) -> Result<OpenAIClient, Box<dyn Error>> {
        let api_endpoint = self.api_endpoint.unwrap_or_else(|| {
            std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned())
        });

        Ok(OpenAIClient {
            api_endpoint,
            api_key: self.api_key,
            organization: self.organization,
            proxy: self.proxy,
            timeout: self.timeout,
            headers: self.headers,
            response_headers: None,
        })
    }
}

impl OpenAIClient {
    pub fn builder() -> OpenAIClientBuilder {
        OpenAIClientBuilder::new()
    }

    async fn build_request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let url = self
            .build_url_with_preserved_query(path)
            .unwrap_or_else(|_| format!("{}/{}", self.api_endpoint, path));

        let client = Client::builder();

        #[cfg(feature = "rustls")]
        let client = client.use_rustls_tls();

        let client = if let Some(timeout) = self.timeout {
            client.timeout(std::time::Duration::from_secs(timeout))
        } else {
            client
        };

        let client = if let Some(proxy) = &self.proxy {
            client.proxy(reqwest::Proxy::all(proxy).unwrap())
        } else {
            client
        };

        let client = client.build().unwrap();

        let mut request = client.request(method, url);

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }

        if let Some(organization) = &self.organization {
            request = request.header("openai-organization", organization);
        }

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        if Self::is_beta(path) {
            request = request.header("OpenAI-Beta", "assistants=v2");
        }

        request
    }

    async fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::ser::Serialize,
    ) -> Result<CallResponse<T>, APIError> {
        let request = self.build_request(Method::POST, path).await;
        let request = request.json(body);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<CallResponse<T>, APIError> {
        let request = self.build_request(Method::GET, path).await;
        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn get_raw(&self, path: &str) -> Result<Bytes, APIError> {
        let request = self.build_request(Method::GET, path).await;
        let response = request.send().await?;
        Ok(response.bytes().await?)
    }

    async fn delete<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<CallResponse<T>, APIError> {
        let request = self.build_request(Method::DELETE, path).await;
        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn post_form<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        form: Form,
    ) -> Result<CallResponse<T>, APIError> {
        let request = self.build_request(Method::POST, path).await;
        let request = request.multipart(form);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn post_form_raw(&self, path: &str, form: Form) -> Result<Bytes, APIError> {
        let request = self.build_request(Method::POST, path).await;
        let request = request.multipart(form);
        let response = request.send().await?;
        Ok(response.bytes().await?)
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<CallResponse<T>, APIError> {
        let status = response.status();
        let headers = response.headers().clone();
        if status.is_success() {
            let text = response.text().await.unwrap_or_else(|_| "".to_string());
            match serde_json::from_str::<T>(&text) {
                Ok(parsed) => Ok(CallResponse {
                    headers,
                    inner: parsed,
                }),
                Err(e) => Err(APIError::CustomError {
                    message: format!("Failed to parse JSON: {e} / response {text}"),
                }),
            }
        } else {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(APIError::CustomError {
                message: format!("{status}: {error_message}"),
            })
        }
    }

    pub async fn completion(
        &self,
        req: CompletionRequest,
    ) -> Result<CallResponse<CompletionResponse>, APIError> {
        self.post("completions", &req).await
    }

    pub async fn edit(&self, req: EditRequest) -> Result<CallResponse<EditResponse>, APIError> {
        self.post("edits", &req).await
    }

    pub async fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> Result<CallResponse<ImageGenerationResponse>, APIError> {
        self.post("images/generations", &req).await
    }

    pub async fn image_edit(
        &self,
        req: ImageEditRequest,
    ) -> Result<CallResponse<ImageEditResponse>, APIError> {
        self.post("images/edits", &req).await
    }

    pub async fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> Result<CallResponse<ImageVariationResponse>, APIError> {
        self.post("images/variations", &req).await
    }

    pub async fn embedding(
        &self,
        req: EmbeddingRequest,
    ) -> Result<CallResponse<EmbeddingResponse>, APIError> {
        self.post("embeddings", &req).await
    }

    pub async fn file_list(&self) -> Result<CallResponse<FileListResponse>, APIError> {
        self.get("files").await
    }

    pub async fn upload_file(
        &self,
        req: FileUploadRequest,
    ) -> Result<CallResponse<FileUploadResponse>, APIError> {
        let form = Self::create_form(&req, "file")?;
        self.post_form("files", form).await
    }

    pub async fn delete_file(
        &self,
        req: FileDeleteRequest,
    ) -> Result<CallResponse<FileDeleteResponse>, APIError> {
        self.delete(&format!("files/{}", req.file_id)).await
    }

    pub async fn retrieve_file(
        &self,
        file_id: String,
    ) -> Result<CallResponse<FileRetrieveResponse>, APIError> {
        self.get(&format!("files/{file_id}")).await
    }

    pub async fn retrieve_file_content(&self, file_id: String) -> Result<Bytes, APIError> {
        self.get_raw(&format!("files/{file_id}/content")).await
    }

    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<CallResponse<ChatCompletionResponse>, APIError> {
        self.post("chat/completions", &req).await
    }

    pub async fn chat_completion_stream(
        &self,
        req: ChatCompletionStreamRequest,
    ) -> Result<impl Stream<Item = ChatCompletionStreamResponse>, APIError> {
        let mut payload = to_value(&req).map_err(|err| APIError::CustomError {
            message: format!("Failed to serialize request: {}", err),
        })?;

        if let Some(obj) = payload.as_object_mut() {
            obj.insert("stream".into(), Value::Bool(true));
        }

        let request = self.build_request(Method::POST, "chat/completions").await;
        let request = request.json(&payload);
        let response = request.send().await?;

        if response.status().is_success() {
            Ok(ChatCompletionStream {
                response: Box::pin(response.bytes_stream()),
                buffer: String::new(),
                first_chunk: true,
            })
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Unknown error"));

            Err(APIError::CustomError {
                message: error_text,
            })
        }
    }

    pub async fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> Result<CallResponse<AudioTranscriptionResponse>, APIError> {
        // https://platform.openai.com/docs/api-reference/audio/createTranscription#audio-createtranscription-response_format
        if let Some(response_format) = &req.response_format {
            if response_format != "json" && response_format != "verbose_json" {
                return Err(APIError::CustomError {
                    message: "response_format must be either 'json' or 'verbose_json' please use audio_transcription_raw".to_string(),
                });
            }
        }
        let form: Form;
        if req.clone().file.is_some() {
            form = Self::create_form(&req, "file")?;
        } else if let Some(bytes) = req.clone().bytes {
            form = Self::create_form_from_bytes(&req, bytes)?;
        } else {
            return Err(APIError::CustomError {
                message: "Either file or bytes must be provided".to_string(),
            });
        }
        self.post_form("audio/transcriptions", form).await
    }

    pub async fn audio_transcription_raw(
        &self,
        req: AudioTranscriptionRequest,
    ) -> Result<Bytes, APIError> {
        // https://platform.openai.com/docs/api-reference/audio/createTranscription#audio-createtranscription-response_format
        if let Some(response_format) = &req.response_format {
            if response_format != "text" && response_format != "srt" && response_format != "vtt" {
                return Err(APIError::CustomError {
                    message: "response_format must be either 'text', 'srt' or 'vtt', please use audio_transcription".to_string(),
                });
            }
        }
        let form: Form;
        if req.clone().file.is_some() {
            form = Self::create_form(&req, "file")?;
        } else if let Some(bytes) = req.clone().bytes {
            form = Self::create_form_from_bytes(&req, bytes)?;
        } else {
            return Err(APIError::CustomError {
                message: "Either file or bytes must be provided".to_string(),
            });
        }
        self.post_form_raw("audio/transcriptions", form).await
    }

    pub async fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> Result<CallResponse<AudioTranslationResponse>, APIError> {
        let form = Self::create_form(&req, "file")?;
        self.post_form("audio/translations", form).await
    }

    pub async fn audio_speech(
        &self,
        req: AudioSpeechRequest,
    ) -> Result<CallResponse<AudioSpeechResponse>, APIError> {
        let request = self.build_request(Method::POST, "audio/speech").await;
        let request = request.json(&req);
        let response = request.send().await?;
        let headers = response.headers().clone();
        let bytes = response.bytes().await?;
        let path = Path::new(req.output.as_str());
        if let Some(parent) = path.parent() {
            match create_dir_all(parent) {
                Ok(_) => {}
                Err(e) => {
                    return Err(APIError::CustomError {
                        message: e.to_string(),
                    })
                }
            }
        }
        match File::create(path) {
            Ok(mut file) => match file.write_all(&bytes) {
                Ok(_) => {}
                Err(e) => {
                    return Err(APIError::CustomError {
                        message: e.to_string(),
                    })
                }
            },
            Err(e) => {
                return Err(APIError::CustomError {
                    message: e.to_string(),
                })
            }
        }

        Ok(CallResponse {
            headers: headers.clone(),
            inner: AudioSpeechResponse { result: true },
        })
    }

    pub async fn create_fine_tuning_job(
        &self,
        req: CreateFineTuningJobRequest,
    ) -> Result<CallResponse<FineTuningJobObject>, APIError> {
        self.post("fine_tuning/jobs", &req).await
    }

    pub async fn list_fine_tuning_jobs(
        &self,
    ) -> Result<CallResponse<FineTuningPagination<FineTuningJobObject>>, APIError> {
        self.get("fine_tuning/jobs").await
    }

    pub async fn list_fine_tuning_job_events(
        &self,
        req: ListFineTuningJobEventsRequest,
    ) -> Result<CallResponse<FineTuningPagination<FineTuningJobEvent>>, APIError> {
        self.get(&format!(
            "fine_tuning/jobs/{}/events",
            req.fine_tuning_job_id
        ))
        .await
    }

    pub async fn retrieve_fine_tuning_job(
        &self,
        req: RetrieveFineTuningJobRequest,
    ) -> Result<CallResponse<FineTuningJobObject>, APIError> {
        self.get(&format!("fine_tuning/jobs/{}", req.fine_tuning_job_id))
            .await
    }

    pub async fn cancel_fine_tuning_job(
        &self,
        req: CancelFineTuningJobRequest,
    ) -> Result<CallResponse<FineTuningJobObject>, APIError> {
        self.post(
            &format!("fine_tuning/jobs/{}/cancel", req.fine_tuning_job_id),
            &req,
        )
        .await
    }

    pub async fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> Result<CallResponse<CreateModerationResponse>, APIError> {
        self.post("moderations", &req).await
    }

    pub async fn create_assistant(
        &self,
        req: AssistantRequest,
    ) -> Result<CallResponse<AssistantObject>, APIError> {
        self.post("assistants", &req).await
    }

    pub async fn retrieve_assistant(
        &self,
        assistant_id: String,
    ) -> Result<CallResponse<AssistantObject>, APIError> {
        self.get(&format!("assistants/{assistant_id}")).await
    }

    pub async fn modify_assistant(
        &self,
        assistant_id: String,
        req: AssistantRequest,
    ) -> Result<CallResponse<AssistantObject>, APIError> {
        self.post(&format!("assistants/{assistant_id}"), &req).await
    }

    pub async fn delete_assistant(
        &self,
        assistant_id: String,
    ) -> Result<CallResponse<common::DeletionStatus>, APIError> {
        self.delete(&format!("assistants/{assistant_id}")).await
    }

    pub async fn list_assistant(
        &self,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<CallResponse<ListAssistant>, APIError> {
        let url = Self::query_params(limit, order, after, before, "assistants".to_string());
        self.get(&url).await
    }

    pub async fn create_assistant_file(
        &self,
        assistant_id: String,
        req: AssistantFileRequest,
    ) -> Result<CallResponse<AssistantFileObject>, APIError> {
        self.post(&format!("assistants/{assistant_id}/files"), &req)
            .await
    }

    pub async fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<CallResponse<AssistantFileObject>, APIError> {
        self.get(&format!("assistants/{assistant_id}/files/{file_id}"))
            .await
    }

    pub async fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<CallResponse<common::DeletionStatus>, APIError> {
        self.delete(&format!("assistants/{assistant_id}/files/{file_id}"))
            .await
    }

    pub async fn list_assistant_file(
        &self,
        assistant_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<CallResponse<ListAssistantFile>, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("assistants/{assistant_id}/files"),
        );
        self.get(&url).await
    }

    pub async fn create_thread(
        &self,
        req: CreateThreadRequest,
    ) -> Result<CallResponse<ThreadObject>, APIError> {
        self.post("threads", &req).await
    }

    pub async fn retrieve_thread(
        &self,
        thread_id: String,
    ) -> Result<CallResponse<ThreadObject>, APIError> {
        self.get(&format!("threads/{thread_id}")).await
    }

    pub async fn modify_thread(
        &self,
        thread_id: String,
        req: ModifyThreadRequest,
    ) -> Result<CallResponse<ThreadObject>, APIError> {
        self.post(&format!("threads/{thread_id}"), &req).await
    }

    pub async fn delete_thread(
        &self,
        thread_id: String,
    ) -> Result<CallResponse<common::DeletionStatus>, APIError> {
        self.delete(&format!("threads/{thread_id}")).await
    }

    pub async fn create_message(
        &self,
        thread_id: String,
        req: CreateMessageRequest,
    ) -> Result<CallResponse<MessageObject>, APIError> {
        self.post(&format!("threads/{thread_id}/messages"), &req)
            .await
    }

    pub async fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<CallResponse<MessageObject>, APIError> {
        self.get(&format!("threads/{thread_id}/messages/{message_id}"))
            .await
    }

    pub async fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        req: ModifyMessageRequest,
    ) -> Result<CallResponse<MessageObject>, APIError> {
        self.post(&format!("threads/{thread_id}/messages/{message_id}"), &req)
            .await
    }

    pub async fn list_messages(
        &self,
        thread_id: String,
    ) -> Result<CallResponse<ListMessage>, APIError> {
        self.get(&format!("threads/{thread_id}/messages")).await
    }

    pub async fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> Result<CallResponse<MessageFileObject>, APIError> {
        self.get(&format!(
            "threads/{thread_id}/messages/{message_id}/files/{file_id}"
        ))
        .await
    }

    pub async fn list_message_file(
        &self,
        thread_id: String,
        message_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<CallResponse<ListMessageFile>, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{thread_id}/messages/{message_id}/files"),
        );
        self.get(&url).await
    }

    pub async fn create_run(
        &self,
        thread_id: String,
        req: CreateRunRequest,
    ) -> Result<CallResponse<RunObject>, APIError> {
        self.post(&format!("threads/{thread_id}/runs"), &req).await
    }

    pub async fn retrieve_run(
        &self,
        thread_id: String,
        run_id: String,
    ) -> Result<CallResponse<RunObject>, APIError> {
        self.get(&format!("threads/{thread_id}/runs/{run_id}"))
            .await
    }

    pub async fn modify_run(
        &self,
        thread_id: String,
        run_id: String,
        req: ModifyRunRequest,
    ) -> Result<CallResponse<RunObject>, APIError> {
        self.post(&format!("threads/{thread_id}/runs/{run_id}"), &req)
            .await
    }

    pub async fn list_run(
        &self,
        thread_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<CallResponse<ListRun>, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{thread_id}/runs"),
        );
        self.get(&url).await
    }

    pub async fn cancel_run(
        &self,
        thread_id: String,
        run_id: String,
    ) -> Result<CallResponse<RunObject>, APIError> {
        self.post(
            &format!("threads/{thread_id}/runs/{run_id}/cancel"),
            &ModifyRunRequest::default(),
        )
        .await
    }

    pub async fn create_thread_and_run(
        &self,
        req: CreateThreadAndRunRequest,
    ) -> Result<CallResponse<RunObject>, APIError> {
        self.post("threads/runs", &req).await
    }

    pub async fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<CallResponse<RunStepObject>, APIError> {
        self.get(&format!(
            "threads/{thread_id}/runs/{run_id}/steps/{step_id}"
        ))
        .await
    }

    pub async fn list_run_step(
        &self,
        thread_id: String,
        run_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<CallResponse<ListRunStep>, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{thread_id}/runs/{run_id}/steps"),
        );
        self.get(&url).await
    }

    pub async fn create_batch(
        &self,
        req: CreateBatchRequest,
    ) -> Result<CallResponse<BatchResponse>, APIError> {
        self.post("batches", &req).await
    }

    pub async fn retrieve_batch(
        &self,
        batch_id: String,
    ) -> Result<CallResponse<BatchResponse>, APIError> {
        self.get(&format!("batches/{batch_id}")).await
    }

    pub async fn cancel_batch(
        &self,
        batch_id: String,
    ) -> Result<CallResponse<BatchResponse>, APIError> {
        self.post(
            &format!("batches/{batch_id}/cancel"),
            &common::EmptyRequestBody {},
        )
        .await
    }

    pub async fn list_batch(
        &self,
        after: Option<String>,
        limit: Option<i64>,
    ) -> Result<CallResponse<ListBatchResponse>, APIError> {
        let url = Self::query_params(limit, None, after, None, "batches".to_string());
        self.get(&url).await
    }

    // Responses API
    pub async fn create_response(
        &self,
        req: CreateResponseRequest,
    ) -> Result<CallResponse<ResponseObject>, APIError> {
        self.post("responses", &req).await
    }

    pub async fn create_response_stream(
        &self,
        req: CreateResponseStreamRequest,
    ) -> Result<impl Stream<Item = ResponseStreamResponse>, APIError> {
        let mut payload = to_value(&req).map_err(|err| APIError::CustomError {
            message: format!("Failed to serialize request: {}", err),
        })?;

        if let Some(obj) = payload.as_object_mut() {
            obj.insert("stream".into(), Value::Bool(true));
        }

        let request = self.build_request(Method::POST, "responses").await;
        let request = request.json(&payload);
        let response = request.send().await?;

        if response.status().is_success() {
            Ok(ResponseStream {
                response: Box::pin(response.bytes_stream()),
                buffer: String::new(),
                first_chunk: true,
            })
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Unknown error"));

            Err(APIError::CustomError {
                message: error_text,
            })
        }
    }

    pub async fn retrieve_response(
        &self,
        response_id: String,
    ) -> Result<CallResponse<ResponseObject>, APIError> {
        self.get(&format!("responses/{response_id}")).await
    }

    pub async fn delete_response(
        &self,
        response_id: String,
    ) -> Result<CallResponse<common::DeletionStatus>, APIError> {
        self.delete(&format!("responses/{response_id}")).await
    }

    pub async fn cancel_response(
        &self,
        response_id: String,
    ) -> Result<CallResponse<ResponseObject>, APIError> {
        self.post(
            &format!("responses/{response_id}/cancel"),
            &common::EmptyRequestBody {},
        )
        .await
    }

    pub async fn list_response_input_items(
        &self,
        response_id: String,
        after: Option<String>,
        limit: Option<i64>,
        order: Option<String>,
    ) -> Result<CallResponse<ListResponses>, APIError> {
        let mut url = format!("responses/{}/input_items", response_id);
        let mut params = vec![];
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        self.get(&url).await
    }

    pub async fn count_response_input_tokens(
        &self,
        req: CountTokensRequest,
    ) -> Result<CallResponse<CountTokensResponse>, APIError> {
        self.post("responses/input_tokens", &req).await
    }

    pub async fn list_models(&self) -> Result<CallResponse<ModelsResponse>, APIError> {
        self.get("models").await
    }

    pub async fn retrieve_model(
        &self,
        model_id: String,
    ) -> Result<CallResponse<ModelResponse>, APIError> {
        self.get(&format!("models/{model_id}")).await
    }

    pub async fn delete_model(
        &self,
        model_id: String,
    ) -> Result<CallResponse<common::DeletionStatus>, APIError> {
        self.delete(&format!("models/{model_id}")).await
    }

    fn build_url_with_preserved_query(&self, path: &str) -> Result<String, url::ParseError> {
        let (base, query_opt) = match self.api_endpoint.split_once('?') {
            Some((b, q)) => (b.trim_end_matches('/'), Some(q)),
            None => (self.api_endpoint.trim_end_matches('/'), None),
        };

        let full_path = format!("{}/{}", base, path.trim_start_matches('/'));
        let mut url = Url::parse(&full_path)?;

        if let Some(query) = query_opt {
            for (k, v) in url::form_urlencoded::parse(query.as_bytes()) {
                url.query_pairs_mut().append_pair(&k, &v);
            }
        }
        Ok(url.to_string())
    }

    fn query_params(
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
        mut url: String,
    ) -> String {
        let mut params = vec![];
        if let Some(limit) = limit {
            params.push(format!("limit={limit}"));
        }
        if let Some(order) = order {
            params.push(format!("order={order}"));
        }
        if let Some(after) = after {
            params.push(format!("after={after}"));
        }
        if let Some(before) = before {
            params.push(format!("before={before}"));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        url
    }

    fn is_beta(path: &str) -> bool {
        path.starts_with("assistants") || path.starts_with("threads")
    }

    fn create_form<T>(req: &T, file_field: &str) -> Result<Form, APIError>
    where
        T: Serialize,
    {
        let json = match serde_json::to_value(req) {
            Ok(json) => json,
            Err(e) => {
                return Err(APIError::CustomError {
                    message: e.to_string(),
                })
            }
        };
        let file_path = if let Value::Object(map) = &json {
            map.get(file_field)
                .and_then(|v| v.as_str())
                .ok_or(APIError::CustomError {
                    message: format!("Field '{file_field}' not found or not a string"),
                })?
        } else {
            return Err(APIError::CustomError {
                message: "Request is not a JSON object".to_string(),
            });
        };

        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(APIError::CustomError {
                    message: e.to_string(),
                })
            }
        };
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                return Err(APIError::CustomError {
                    message: e.to_string(),
                })
            }
        }

        let mut form =
            Form::new().part("file", Part::bytes(buffer).file_name(file_path.to_string()));

        if let Value::Object(map) = json {
            for (key, value) in map.into_iter() {
                if key != file_field {
                    match value {
                        Value::String(s) => {
                            form = form.text(key, s);
                        }
                        Value::Number(n) => {
                            form = form.text(key, n.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(form)
    }

    fn create_form_from_bytes<T>(req: &T, bytes: Vec<u8>) -> Result<Form, APIError>
    where
        T: Serialize,
    {
        let json = match serde_json::to_value(req) {
            Ok(json) => json,
            Err(e) => {
                return Err(APIError::CustomError {
                    message: e.to_string(),
                })
            }
        };

        let mut form = Form::new().part("file", Part::bytes(bytes.clone()).file_name("file.mp3"));

        if let Value::Object(map) = json {
            for (key, value) in map.into_iter() {
                match value {
                    Value::String(s) => {
                        form = form.text(key, s);
                    }
                    Value::Number(n) => {
                        form = form.text(key, n.to_string());
                    }
                    _ => {}
                }
            }
        }

        Ok(form)
    }
}
