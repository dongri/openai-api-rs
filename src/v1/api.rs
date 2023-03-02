use crate::v1::chat_completion::{ChatCompletionRequest, ChatCompletionResponse};
use crate::v1::completion::{CompletionRequest, CompletionResponse};
use crate::v1::edit::{EditRequest, EditResponse};
use crate::v1::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::v1::file::{
    FileDeleteRequest, FileDeleteResponse, FileListResponse, FileRetrieveContentRequest,
    FileRetrieveContentResponse, FileRetrieveRequest, FileRetrieveResponse, FileUploadRequest,
    FileUploadResponse,
};
use crate::v1::image::{
    ImageEditRequest, ImageEditResponse, ImageGenerationRequest, ImageGenerationResponse,
    ImageVariationRequest, ImageVariationResponse,
};
use reqwest::Response;

const APU_URL_V1: &str = "https://api.openai.com/v1";

pub struct Client {
    pub api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn post<T: serde::ser::Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{APU_URL_V1}{path}");
        let res = client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                reqwest::header::AUTHORIZATION,
                "Bearer ".to_owned() + &self.api_key,
            )
            .json(&params)
            .send()
            .await;
        match res {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{}: {}", res.status(), res.text().await.unwrap()),
                ))),
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get(&self, path: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{APU_URL_V1}{path}");
        let res = client
            .get(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                reqwest::header::AUTHORIZATION,
                "Bearer ".to_owned() + &self.api_key,
            )
            .send()
            .await;
        match res {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{}: {}", res.status(), res.text().await.unwrap()),
                ))),
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn delete(&self, path: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{APU_URL_V1}{path}");
        let res = client
            .delete(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                reqwest::header::AUTHORIZATION,
                "Bearer ".to_owned() + &self.api_key,
            )
            .send()
            .await;
        match res {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{}: {}", res.status(), res.text().await.unwrap()),
                ))),
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn completion(
        &self,
        req: CompletionRequest,
    ) -> Result<CompletionResponse, Box<dyn std::error::Error>> {
        let res = self.post("/completions", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<CompletionResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn edit(&self, req: EditRequest) -> Result<EditResponse, Box<dyn std::error::Error>> {
        let res = self.post("/edits", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<EditResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn image_generation(
        &self,
        req: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, Box<dyn std::error::Error>> {
        let res = self.post("/images/generations", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<ImageGenerationResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn image_edit(
        &self,
        req: ImageEditRequest,
    ) -> Result<ImageEditResponse, Box<dyn std::error::Error>> {
        let res = self.post("/images/edits", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<ImageEditResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn image_variation(
        &self,
        req: ImageVariationRequest,
    ) -> Result<ImageVariationResponse, Box<dyn std::error::Error>> {
        let res = self.post("/images/variations", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<ImageVariationResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn embedding(
        &self,
        req: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, Box<dyn std::error::Error>> {
        let res = self.post("/embeddings", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<EmbeddingResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn file_list(&self) -> Result<FileListResponse, Box<dyn std::error::Error>> {
        let res = self.get("/files").await;
        match res {
            Ok(res) => {
                let r = res.json::<FileListResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn file_upload(
        &self,
        req: FileUploadRequest,
    ) -> Result<FileUploadResponse, Box<dyn std::error::Error>> {
        let res = self.post("/files", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<FileUploadResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn file_delete(
        &self,
        req: FileDeleteRequest,
    ) -> Result<FileDeleteResponse, Box<dyn std::error::Error>> {
        let res = self.delete(&format!("{}/{}", "/files", req.file_id)).await;
        match res {
            Ok(res) => {
                let r = res.json::<FileDeleteResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn file_retrieve(
        &self,
        req: FileRetrieveRequest,
    ) -> Result<FileRetrieveResponse, Box<dyn std::error::Error>> {
        let res = self.get(&format!("{}/{}", "/files", req.file_id)).await;
        match res {
            Ok(res) => {
                let r = res.json::<FileRetrieveResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn file_retrieve_content(
        &self,
        req: FileRetrieveContentRequest,
    ) -> Result<FileRetrieveContentResponse, Box<dyn std::error::Error>> {
        let res = self
            .get(&format!("{}/{}/content", "/files", req.file_id))
            .await;
        match res {
            Ok(res) => {
                let r = res.json::<FileRetrieveContentResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
        let res = self.post("/chat/completions", &req).await;
        match res {
            Ok(res) => {
                let r = res.json::<ChatCompletionResponse>().await?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }
}
