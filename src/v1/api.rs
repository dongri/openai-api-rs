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

use minreq::Response;

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

    pub fn post<T: serde::ser::Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );

        let mut request = minreq::post(url)
            .with_header("Content-Type", "application/json")
            .with_header("Authorization", format!("Bearer {}", self.api_key));

        if let Some(organization) = &self.organization {
            request = request.with_header("openai-organization", organization);
        }

        let res = request.with_json(params).unwrap().send();

        match res {
            Ok(res) => {
                if (200..=299).contains(&res.status_code) {
                    Ok(res)
                } else {
                    Err(APIError {
                        message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
                    })
                }
            }
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn get(&self, path: &str) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );

        let mut request = minreq::get(url)
            .with_header("Content-Type", "application/json")
            .with_header("Authorization", format!("Bearer {}", self.api_key));

        if let Some(organization) = &self.organization {
            request = request.with_header("openai-organization", organization);
        }

        let res = request.send();

        match res {
            Ok(res) => {
                if (200..=299).contains(&res.status_code) {
                    Ok(res)
                } else {
                    Err(APIError {
                        message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
                    })
                }
            }
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn delete(&self, path: &str) -> Result<Response, APIError> {
        let url = format!(
            "{api_endpoint}{path}",
            api_endpoint = self.api_endpoint,
            path = path
        );

        let mut request = minreq::delete(url)
            .with_header("Content-Type", "application/json")
            .with_header("Authorization", format!("Bearer {}", self.api_key));

        if let Some(organization) = &self.organization {
            request = request.with_header("openai-organization", organization);
        }

        let res = request.send();

        match res {
            Ok(res) => {
                if (200..=299).contains(&res.status_code) {
                    Ok(res)
                } else {
                    Err(APIError {
                        message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
                    })
                }
            }
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn completion(&self, req: CompletionRequest) -> Result<CompletionResponse, APIError> {
        let res = self.post("/completions", &req)?;
        let r = res.json::<CompletionResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn edit(&self, req: EditRequest) -> Result<EditResponse, APIError> {
        let res = self.post("/edits", &req)?;
        let r = res.json::<EditResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, APIError> {
        let res = self.post("/images/generations", &req)?;
        let r = res.json::<ImageGenerationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn image_edit(&self, req: ImageEditRequest) -> Result<ImageEditResponse, APIError> {
        let res = self.post("/images/edits", &req)?;
        let r = res.json::<ImageEditResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> Result<ImageVariationResponse, APIError> {
        let res = self.post("/images/variations", &req)?;
        let r = res.json::<ImageVariationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn embedding(&self, req: EmbeddingRequest) -> Result<EmbeddingResponse, APIError> {
        let res = self.post("/embeddings", &req)?;
        let r = res.json::<EmbeddingResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_list(&self) -> Result<FileListResponse, APIError> {
        let res = self.get("/files")?;
        let r = res.json::<FileListResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_upload(&self, req: FileUploadRequest) -> Result<FileUploadResponse, APIError> {
        let res = self.post("/files", &req)?;
        let r = res.json::<FileUploadResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_delete(&self, req: FileDeleteRequest) -> Result<FileDeleteResponse, APIError> {
        let res = self.delete(&format!("{}/{}", "/files", req.file_id))?;
        let r = res.json::<FileDeleteResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_retrieve(
        &self,
        req: FileRetrieveRequest,
    ) -> Result<FileRetrieveResponse, APIError> {
        let res = self.get(&format!("{}/{}", "/files", req.file_id))?;
        let r = res.json::<FileRetrieveResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn file_retrieve_content(
        &self,
        req: FileRetrieveContentRequest,
    ) -> Result<FileRetrieveContentResponse, APIError> {
        let res = self.get(&format!("{}/{}/content", "/files", req.file_id))?;
        let r = res.json::<FileRetrieveContentResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, APIError> {
        let res = self.post("/chat/completions", &req)?;
        let r = res.json::<ChatCompletionResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn audio_transcription(
        &self,
        req: AudioTranscriptionRequest,
    ) -> Result<AudioTranscriptionResponse, APIError> {
        let res = self.post("/audio/transcriptions", &req)?;
        let r = res.json::<AudioTranscriptionResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn audio_translation(
        &self,
        req: AudioTranslationRequest,
    ) -> Result<AudioTranslationResponse, APIError> {
        let res = self.post("/audio/translations", &req)?;
        let r = res.json::<AudioTranslationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_fine_tune(
        &self,
        req: CreateFineTuneRequest,
    ) -> Result<CreateFineTuneResponse, APIError> {
        let res = self.post("/fine-tunes", &req)?;
        let r = res.json::<CreateFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_fine_tune(&self) -> Result<ListFineTuneResponse, APIError> {
        let res = self.get("/fine-tunes")?;
        let r = res.json::<ListFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn retrieve_fine_tune(
        &self,
        req: RetrieveFineTuneRequest,
    ) -> Result<RetrieveFineTuneResponse, APIError> {
        let res = self.get(&format!("/fine_tunes/{}", req.fine_tune_id))?;
        let r = res.json::<RetrieveFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn cancel_fine_tune(
        &self,
        req: CancelFineTuneRequest,
    ) -> Result<CancelFineTuneResponse, APIError> {
        let res = self.post(&format!("/fine_tunes/{}/cancel", req.fine_tune_id), &req)?;
        let r = res.json::<CancelFineTuneResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn list_fine_tune_events(
        &self,
        req: ListFineTuneEventsRequest,
    ) -> Result<ListFineTuneEventsResponse, APIError> {
        let res = self.get(&format!("/fine-tunes/{}/events", req.fine_tune_id))?;
        let r = res.json::<ListFineTuneEventsResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn delete_fine_tune(
        &self,
        req: DeleteFineTuneModelRequest,
    ) -> Result<DeleteFineTuneModelResponse, APIError> {
        let res = self.delete(&format!("/models/{}", req.model_id))?;
        let r = res.json::<DeleteFineTuneModelResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    pub fn create_moderation(
        &self,
        req: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, APIError> {
        let res = self.post("/moderations", &req)?;
        let r = res.json::<CreateModerationResponse>();
        match r {
            Ok(r) => Ok(r),
            Err(e) => Err(self.new_error(e)),
        }
    }

    fn new_error(&self, err: minreq::Error) -> APIError {
        APIError {
            message: err.to_string(),
        }
    }
}
