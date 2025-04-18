use crate::v1::assistant::{
    AssistantFileObject, AssistantFileRequest, AssistantObject, AssistantRequest, DeletionStatus,
    ListAssistant, ListAssistantFile,
};
use crate::v1::audio::{
    AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest, AudioTranscriptionResponse,
    AudioTranslationRequest, AudioTranslationResponse,
};
use crate::v1::batch::{BatchResponse, CreateBatchRequest, ListBatchResponse};
use crate::v1::chat_completion::{ChatCompletionRequest, ChatCompletionResponse};
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
use crate::v1::moderation::{CreateModerationRequest, CreateModerationResponse};
use crate::v1::run::{
    CreateRunRequest, CreateThreadAndRunRequest, ListRun, ListRunStep, ModifyRunRequest, RunObject,
    RunStepObject,
};
use crate::v1::thread::{CreateThreadRequest, ModifyThreadRequest, ThreadObject};

use bytes::Bytes;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Method, Response};
use serde::Serialize;
use serde_json::Value;

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
    api_version: Option<String>,
    organization: Option<String>,
    proxy: Option<String>,
    timeout: Option<u64>,
    headers: Option<HeaderMap>,
}

#[derive(Debug)]
pub struct OpenAIClient {
    api_endpoint: String,
    api_key: Option<String>,
    api_version: Option<String>,
    organization: Option<String>,
    proxy: Option<String>,
    timeout: Option<u64>,
    pub headers: Option<HeaderMap>,
}

impl OpenAIClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_api_version(mut self, api_version: impl Into<String>) -> Self {
        self.api_version = Some(api_version.into());
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
            api_version: self.api_version,
            api_key: self.api_key,
            organization: self.organization,
            proxy: self.proxy,
            timeout: self.timeout,
            headers: self.headers,
        })
    }
}

impl OpenAIClient {
    pub fn builder() -> OpenAIClientBuilder {
        OpenAIClientBuilder::new()
    }

    async fn build_request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!(
            "{}/{}?api-version={}",
            self.api_endpoint,
            path,
            self.api_version.as_deref().unwrap_or("v1")
        );

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

        if self.api_key.is_some() {
            request = request.header(
                "Authorization",
                format!("Bearer {}", self.api_key.as_ref().unwrap()),
            );
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
        &mut self,
        path: &str,
        body: &impl serde::ser::Serialize,
    ) -> Result<T, APIError> {
        let request_builder = self.build_request(Method::POST, path).await;
        let body_json = serde_json::to_string(body).map_err(|e| APIError::CustomError {
            message: format!("Failed to serialize body: {}", e),
        })?;
        let request_builder = request_builder.json(body);

        // 💡 Convert to request to inspect it before sending
        let client = request_builder
            .try_clone()
            .expect("Cannot clone request builder")
            .build()
            .expect("Failed to build request");

        // 🔍 Debug log: URL, headers, and optionally body
        tracing::info!("🔵 URL: {}", client.url());
        tracing::info!("🟢 Headers:\n{:#?}", client.headers());
        tracing::info!("🔴 Body:\n{:#?}", body_json);
        let response = request_builder.send().await?;
        tracing::info!("Response: {:?}", response);
        self.handle_response(response).await
    }

    async fn get<T: serde::de::DeserializeOwned>(&mut self, path: &str) -> Result<T, APIError> {
        let request = self.build_request(Method::GET, path).await;
        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn get_raw(&self, path: &str) -> Result<Bytes, APIError> {
        let request = self.build_request(Method::GET, path).await;
        let response = request.send().await?;
        Ok(response.bytes().await?)
    }

    async fn delete<T: serde::de::DeserializeOwned>(&mut self, path: &str) -> Result<T, APIError> {
        let request = self.build_request(Method::DELETE, path).await;
        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn post_form<T: serde::de::DeserializeOwned>(
        &mut self,
        path: &str,
        form: Form,
    ) -> Result<T, APIError> {
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
        &mut self,
        response: Response,
    ) -> Result<T, APIError> {
        let status = response.status();
        let headers = response.headers().clone();
        if status.is_success() {
            let text = response.text().await.unwrap_or_else(|_| "".to_string());
            match serde_json::from_str::<T>(&text) {
                Ok(parsed) => {
                    self.headers = Some(headers);
                    Ok(parsed)
                }
                Err(e) => Err(APIError::CustomError {
                    message: format!("Failed to parse JSON: {} / response {}", e, text),
                }),
            }
        } else {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(APIError::CustomError {
                message: format!("{}: {}", status, error_message),
            })
        }
    }

    pub async fn completion(
        &mut self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse, APIError> {
        self.post("completions", &req).await
    }

    pub async fn edit(&mut self, req: EditRequest) -> Result<EditResponse, APIError> {
        self.post("edits", &req).await
    }

    pub async fn image_generation(
        &mut self,
        req: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, APIError> {
        self.post("images/generations", &req).await
    }

    pub async fn image_edit(
        &mut self,
        req: ImageEditRequest,
    ) -> Result<ImageEditResponse, APIError> {
        self.post("images/edits", &req).await
    }

    pub async fn image_variation(
        &mut self,
        req: ImageVariationRequest,
    ) -> Result<ImageVariationResponse, APIError> {
        self.post("images/variations", &req).await
    }

    pub async fn embedding(
        &mut self,
        req: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, APIError> {
        self.post("embeddings", &req).await
    }

    pub async fn file_list(&mut self) -> Result<FileListResponse, APIError> {
        self.get("files").await
    }

    pub async fn upload_file(
        &mut self,
        req: FileUploadRequest,
    ) -> Result<FileUploadResponse, APIError> {
        let form = Self::create_form(&req, "file")?;
        self.post_form("files", form).await
    }

    pub async fn delete_file(
        &mut self,
        req: FileDeleteRequest,
    ) -> Result<FileDeleteResponse, APIError> {
        self.delete(&format!("files/{}", req.file_id)).await
    }

    pub async fn retrieve_file(
        &mut self,
        file_id: String,
    ) -> Result<FileRetrieveResponse, APIError> {
        self.get(&format!("files/{}", file_id)).await
    }

    pub async fn retrieve_file_content(&self, file_id: String) -> Result<Bytes, APIError> {
        self.get_raw(&format!("files/{}/content", file_id)).await
    }

    pub async fn chat_completion(
        &mut self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, APIError> {
        self.post("chat/completions", &req).await
    }

    pub async fn audio_transcription(
        &mut self,
        req: AudioTranscriptionRequest,
    ) -> Result<AudioTranscriptionResponse, APIError> {
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
        &mut self,
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
        &mut self,
        req: AudioTranslationRequest,
    ) -> Result<AudioTranslationResponse, APIError> {
        let form = Self::create_form(&req, "file")?;
        self.post_form("audio/translations", form).await
    }

    pub async fn audio_speech(
        &mut self,
        req: AudioSpeechRequest,
    ) -> Result<AudioSpeechResponse, APIError> {
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

        Ok(AudioSpeechResponse {
            result: true,
            headers: Some(headers),
        })
    }

    pub async fn create_fine_tuning_job(
        &mut self,
        req: CreateFineTuningJobRequest,
    ) -> Result<FineTuningJobObject, APIError> {
        self.post("fine_tuning/jobs", &req).await
    }

    pub async fn list_fine_tuning_jobs(
        &mut self,
    ) -> Result<FineTuningPagination<FineTuningJobObject>, APIError> {
        self.get("fine_tuning/jobs").await
    }

    pub async fn list_fine_tuning_job_events(
        &mut self,
        req: ListFineTuningJobEventsRequest,
    ) -> Result<FineTuningPagination<FineTuningJobEvent>, APIError> {
        self.get(&format!(
            "fine_tuning/jobs/{}/events",
            req.fine_tuning_job_id
        ))
        .await
    }

    pub async fn retrieve_fine_tuning_job(
        &mut self,
        req: RetrieveFineTuningJobRequest,
    ) -> Result<FineTuningJobObject, APIError> {
        self.get(&format!("fine_tuning/jobs/{}", req.fine_tuning_job_id))
            .await
    }

    pub async fn cancel_fine_tuning_job(
        &mut self,
        req: CancelFineTuningJobRequest,
    ) -> Result<FineTuningJobObject, APIError> {
        self.post(
            &format!("fine_tuning/jobs/{}/cancel", req.fine_tuning_job_id),
            &req,
        )
        .await
    }

    pub async fn create_moderation(
        &mut self,
        req: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, APIError> {
        self.post("moderations", &req).await
    }

    pub async fn create_assistant(
        &mut self,
        req: AssistantRequest,
    ) -> Result<AssistantObject, APIError> {
        self.post("assistants", &req).await
    }

    pub async fn retrieve_assistant(
        &mut self,
        assistant_id: String,
    ) -> Result<AssistantObject, APIError> {
        self.get(&format!("assistants/{}", assistant_id)).await
    }

    pub async fn modify_assistant(
        &mut self,
        assistant_id: String,
        req: AssistantRequest,
    ) -> Result<AssistantObject, APIError> {
        self.post(&format!("assistants/{}", assistant_id), &req)
            .await
    }

    pub async fn delete_assistant(
        &mut self,
        assistant_id: String,
    ) -> Result<DeletionStatus, APIError> {
        self.delete(&format!("assistants/{}", assistant_id)).await
    }

    pub async fn list_assistant(
        &mut self,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListAssistant, APIError> {
        let url = Self::query_params(limit, order, after, before, "assistants".to_string());
        self.get(&url).await
    }

    pub async fn create_assistant_file(
        &mut self,
        assistant_id: String,
        req: AssistantFileRequest,
    ) -> Result<AssistantFileObject, APIError> {
        self.post(&format!("assistants/{}/files", assistant_id), &req)
            .await
    }

    pub async fn retrieve_assistant_file(
        &mut self,
        assistant_id: String,
        file_id: String,
    ) -> Result<AssistantFileObject, APIError> {
        self.get(&format!("assistants/{}/files/{}", assistant_id, file_id))
            .await
    }

    pub async fn delete_assistant_file(
        &mut self,
        assistant_id: String,
        file_id: String,
    ) -> Result<DeletionStatus, APIError> {
        self.delete(&format!("assistants/{}/files/{}", assistant_id, file_id))
            .await
    }

    pub async fn list_assistant_file(
        &mut self,
        assistant_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListAssistantFile, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("assistants/{}/files", assistant_id),
        );
        self.get(&url).await
    }

    pub async fn create_thread(
        &mut self,
        req: CreateThreadRequest,
    ) -> Result<ThreadObject, APIError> {
        self.post("threads", &req).await
    }

    pub async fn retrieve_thread(&mut self, thread_id: String) -> Result<ThreadObject, APIError> {
        self.get(&format!("threads/{}", thread_id)).await
    }

    pub async fn modify_thread(
        &mut self,
        thread_id: String,
        req: ModifyThreadRequest,
    ) -> Result<ThreadObject, APIError> {
        self.post(&format!("threads/{}", thread_id), &req).await
    }

    pub async fn delete_thread(&mut self, thread_id: String) -> Result<DeletionStatus, APIError> {
        self.delete(&format!("threads/{}", thread_id)).await
    }

    pub async fn create_message(
        &mut self,
        thread_id: String,
        req: CreateMessageRequest,
    ) -> Result<MessageObject, APIError> {
        self.post(&format!("threads/{}/messages", thread_id), &req)
            .await
    }

    pub async fn retrieve_message(
        &mut self,
        thread_id: String,
        message_id: String,
    ) -> Result<MessageObject, APIError> {
        self.get(&format!("threads/{}/messages/{}", thread_id, message_id))
            .await
    }

    pub async fn modify_message(
        &mut self,
        thread_id: String,
        message_id: String,
        req: ModifyMessageRequest,
    ) -> Result<MessageObject, APIError> {
        self.post(
            &format!("threads/{}/messages/{}", thread_id, message_id),
            &req,
        )
        .await
    }

    pub async fn list_messages(&mut self, thread_id: String) -> Result<ListMessage, APIError> {
        self.get(&format!("threads/{}/messages", thread_id)).await
    }

    pub async fn retrieve_message_file(
        &mut self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> Result<MessageFileObject, APIError> {
        self.get(&format!(
            "threads/{}/messages/{}/files/{}",
            thread_id, message_id, file_id
        ))
        .await
    }

    pub async fn list_message_file(
        &mut self,
        thread_id: String,
        message_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListMessageFile, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{}/messages/{}/files", thread_id, message_id),
        );
        self.get(&url).await
    }

    pub async fn create_run(
        &mut self,
        thread_id: String,
        req: CreateRunRequest,
    ) -> Result<RunObject, APIError> {
        self.post(&format!("threads/{}/runs", thread_id), &req)
            .await
    }

    pub async fn retrieve_run(
        &mut self,
        thread_id: String,
        run_id: String,
    ) -> Result<RunObject, APIError> {
        self.get(&format!("threads/{}/runs/{}", thread_id, run_id))
            .await
    }

    pub async fn modify_run(
        &mut self,
        thread_id: String,
        run_id: String,
        req: ModifyRunRequest,
    ) -> Result<RunObject, APIError> {
        self.post(&format!("threads/{}/runs/{}", thread_id, run_id), &req)
            .await
    }

    pub async fn list_run(
        &mut self,
        thread_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRun, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{}/runs", thread_id),
        );
        self.get(&url).await
    }

    pub async fn cancel_run(
        &mut self,
        thread_id: String,
        run_id: String,
    ) -> Result<RunObject, APIError> {
        self.post(
            &format!("threads/{}/runs/{}/cancel", thread_id, run_id),
            &ModifyRunRequest::default(),
        )
        .await
    }

    pub async fn create_thread_and_run(
        &mut self,
        req: CreateThreadAndRunRequest,
    ) -> Result<RunObject, APIError> {
        self.post("threads/runs", &req).await
    }

    pub async fn retrieve_run_step(
        &mut self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<RunStepObject, APIError> {
        self.get(&format!(
            "threads/{}/runs/{}/steps/{}",
            thread_id, run_id, step_id
        ))
        .await
    }

    pub async fn list_run_step(
        &mut self,
        thread_id: String,
        run_id: String,
        limit: Option<i64>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<ListRunStep, APIError> {
        let url = Self::query_params(
            limit,
            order,
            after,
            before,
            format!("threads/{}/runs/{}/steps", thread_id, run_id),
        );
        self.get(&url).await
    }

    pub async fn create_batch(
        &mut self,
        req: CreateBatchRequest,
    ) -> Result<BatchResponse, APIError> {
        self.post("batches", &req).await
    }

    pub async fn retrieve_batch(&mut self, batch_id: String) -> Result<BatchResponse, APIError> {
        self.get(&format!("batches/{}", batch_id)).await
    }

    pub async fn cancel_batch(&mut self, batch_id: String) -> Result<BatchResponse, APIError> {
        self.post(
            &format!("batches/{}/cancel", batch_id),
            &common::EmptyRequestBody {},
        )
        .await
    }

    pub async fn list_batch(
        &mut self,
        after: Option<String>,
        limit: Option<i64>,
    ) -> Result<ListBatchResponse, APIError> {
        let url = Self::query_params(limit, None, after, None, "batches".to_string());
        self.get(&url).await
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
            params.push(format!("limit={}", limit));
        }
        if let Some(order) = order {
            params.push(format!("order={}", order));
        }
        if let Some(after) = after {
            params.push(format!("after={}", after));
        }
        if let Some(before) = before {
            params.push(format!("before={}", before));
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
                    message: format!("Field '{}' not found or not a string", file_field),
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
