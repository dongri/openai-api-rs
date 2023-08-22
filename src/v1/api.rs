use crate::v1::audio::{
    AudioTranscriptionRequest, AudioTranscriptionResponse, AudioTranslationRequest,
    AudioTranslationResponse,
};
use crate::v1::chat_completion::{ChatCompletionRequest, ChatCompletionResponse};
use crate::v1::completion::{CompletionRequest, CompletionResponse};
use crate::v1::edit::{EditRequest, EditResponse};
use crate::v1::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::v1::error::APIError;
use crate::v1::file::{
    FileDeleteRequest, FileDeleteResponse, FileListResponse, FileRetrieveContentRequest,
    FileRetrieveContentResponse, FileRetrieveRequest, FileRetrieveResponse, FileUploadRequest,
    FileUploadResponse,
};
use crate::v1::fine_tune::{
    CancelFineTuneRequest, CancelFineTuneResponse, CreateFineTuneRequest, CreateFineTuneResponse,
    DeleteFineTuneModelRequest, DeleteFineTuneModelResponse, ListFineTuneEventsRequest,
    ListFineTuneEventsResponse, ListFineTuneResponse, RetrieveFineTuneRequest,
    RetrieveFineTuneResponse,
};
use crate::v1::image::{
    ImageEditRequest, ImageEditResponse, ImageGenerationRequest, ImageGenerationResponse,
    ImageVariationRequest, ImageVariationResponse,
};
use crate::v1::moderation::{CreateModerationRequest, CreateModerationResponse};

use reqwest::Response;

const API_URL_V1: &str = "https://api.openai.com/v1";

pub struct Client {
    pub api_endpoint: String,
    pub api_key: String,
    pub organization: Option<String>,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        Self::new_with_endpoint(endpoint, api_key)
    }

    pub fn new_with_endpoint(api_endpoint: String, api_key: String) -> Self {
        Self {
            api_endpoint,
            api_key,
            organization: None,
        }
    }

    pub fn new_with_organization(api_key: String, organization: String) -> Self {
        let endpoint = std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
        Self {
            api_endpoint: endpoint,
            api_key,
            organization: organization.into(),
        }
    }

    pub async fn client_builder(&self) -> Result<reqwest::Client, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&("Bearer ".to_owned() + &self.api_key))
                .unwrap(),
        );
        match &self.organization {
            Some(organization) => headers.insert(
                reqwest::header::HeaderName::from_static("openai-organization"),
                reqwest::header::HeaderValue::from_str(organization).unwrap(),
            ),
            None => None,
        };
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(client)
    }

    pub async fn post<T: serde::ser::Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );
        let client = match self.client_builder().await {
            Ok(c) => c,
            Err(e) => return Err(self.new_error(e)),
        };
        let res = client.post(&url).json(&params).send().await;
        match res {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => Err(APIError {
                    message: format!("{}: {}", res.status(), res.text().await.unwrap()),
                }),
            },
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn get(&self, path: &str) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );
        let client = match self.client_builder().await {
            Ok(c) => c,
            Err(e) => return Err(self.new_error(e)),
        };
        let res = client.get(&url).send().await;
        match res {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => Err(APIError {
                    message: format!("{}: {}", res.status(), res.text().await.unwrap()),
                }),
            },
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn delete(&self, path: &str) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );
        let client = match self.client_builder().await {
            Ok(c) => c,
            Err(e) => return Err(self.new_error(e)),
        };
        let res = client.delete(&url).send().await;
        match res {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => Err(APIError {
                    message: format!("{}: {}", res.status(), res.text().await.unwrap()),
                }),
            },
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn completion(&self, req: CompletionRequest) -> Result<CompletionResponse, APIError> {
        let res = self.post("/completions", &req).await?;
        let r = res.json::<CompletionResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn edit(&self, req: EditRequest) -> Result<EditResponse, APIError> {
        let res = self.post("/edits", &req).await?;
        let r = res.json::<EditResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, APIError> {
        let res = self.post("/images/generations", &req).await?;
        let r = res.json::<ImageGenerationResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn image_edit(&self, req: ImageEditRequest) -> Result<ImageEditResponse, APIError> {
        let res = self.post("/images/edits", &req).await?;
        let r = res.json::<ImageEditResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> Result<ImageVariationResponse, APIError> {
        let res = self.post("/images/variations", &req).await?;
        let r = res.json::<ImageVariationResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn embedding(&self, req: EmbeddingRequest) -> Result<EmbeddingResponse, APIError> {
        let res = self.post("/embeddings", &req).await?;
        let r = res.json::<EmbeddingResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn file_list(&self) -> Result<FileListResponse, APIError> {
        let res = self.get("/files").await?;
        let r = res.json::<FileListResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn file_upload(
        &self,
        req: FileUploadRequest,
    ) -> Result<FileUploadResponse, APIError> {
        let res = self.post("/files", &req).await?;
        let r = res.json::<FileUploadResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn file_delete(
        &self,
        req: FileDeleteRequest,
    ) -> Result<FileDeleteResponse, APIError> {
        let res = self
            .delete(&format!("{}/{}", "/files", req.file_id))
            .await?;
        let r = res.json::<FileDeleteResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn file_retrieve(
        &self,
        req: FileRetrieveRequest,
    ) -> Result<FileRetrieveResponse, APIError> {
        let res = self.get(&format!("{}/{}", "/files", req.file_id)).await?;
        let r = res.json::<FileRetrieveResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn file_retrieve_content(
        &self,
        req: FileRetrieveContentRequest,
    ) -> Result<FileRetrieveContentResponse, APIError> {
        let res = self
            .get(&format!("{}/{}/content", "/files", req.file_id))
            .await?;
        let r = res.json::<FileRetrieveContentResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, APIError> {
        let res = self.post("/chat/completions", &req).await?;
        let r = res.json::<ChatCompletionResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> Result<AudioTranscriptionResponse, APIError> {
        let res = self.post("/audio/transcriptions", &req).await?;
        let r = res.json::<AudioTranscriptionResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> Result<AudioTranslationResponse, APIError> {
        let res = self.post("/audio/translations", &req).await?;
        let r = res.json::<AudioTranslationResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn create_fine_tune(
        &self,
        req: CreateFineTuneRequest,
    ) -> Result<CreateFineTuneResponse, APIError> {
        let res = self.post("/fine-tunes", &req).await?;
        let r = res.json::<CreateFineTuneResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn list_fine_tune(&self) -> Result<ListFineTuneResponse, APIError> {
        let res = self.get("/fine-tunes").await?;
        let r = res.json::<ListFineTuneResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn retrieve_fine_tune(
        &self,
        req: RetrieveFineTuneRequest,
    ) -> Result<RetrieveFineTuneResponse, APIError> {
        let res = self
            .get(&format!("/fine_tunes/{}", req.fine_tune_id))
            .await?;
        let r = res.json::<RetrieveFineTuneResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn cancel_fine_tune(
        &self,
        req: CancelFineTuneRequest,
    ) -> Result<CancelFineTuneResponse, APIError> {
        let res = self
            .post(&format!("/fine_tunes/{}/cancel", req.fine_tune_id), &req)
            .await?;
        let r = res.json::<CancelFineTuneResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn list_fine_tune_events(
        &self,
        req: ListFineTuneEventsRequest,
    ) -> Result<ListFineTuneEventsResponse, APIError> {
        let res = self
            .get(&format!("/fine-tunes/{}/events", req.fine_tune_id))
            .await?;
        let r = res.json::<ListFineTuneEventsResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn delete_fine_tune(
        &self,
        req: DeleteFineTuneModelRequest,
    ) -> Result<DeleteFineTuneModelResponse, APIError> {
        let res = self.delete(&format!("/models/{}", req.model_id)).await?;
        let r = res.json::<DeleteFineTuneModelResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub async fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, APIError> {
        let res = self.post("/moderations", &req).await?;
        let r = res.json::<CreateModerationResponse>().await;
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    fn new_error(&self, err: reqwest::Error) -> APIError {
        APIError {
            message: err.to_string(),
        }
    }
}
