use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateFineTuneRequest {
    pub training_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_loss_weight: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_classification_metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_n_classes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_positive_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_betas: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFineTuneResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub events: Vec<FineTuneEvent>,
    pub fine_tuned_model: Option<FineTunedModel>,
    pub hyperparams: HyperParams,
    pub organization_id: String,
    pub result_files: Vec<ResultFile>,
    pub status: String,
    pub validation_files: Vec<ValidationFile>,
    pub training_files: Vec<TrainingFile>,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct FineTuneEvent {
    pub object: String,
    pub created_at: i64,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct FineTunedModel {
    pub id: String,
    pub object: String,
    pub model_details: ModelDetails,
}

#[derive(Debug, Deserialize)]
pub struct ModelDetails {
    pub architecture: String,
    pub created_at: i64,
    pub id: String,
    pub object: String,
    pub prompt: String,
    pub samples_seen: i64,
}

#[derive(Debug, Deserialize)]
pub struct HyperParams {
    pub batch_size: i32,
    pub learning_rate_multiplier: f32,
    pub n_epochs: i32,
    pub prompt_loss_weight: f32,
}

#[derive(Debug, Deserialize)]
pub struct ResultFile {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidationFile {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct TrainingFile {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct ListFineTuneResponse {
    pub object: String,
    pub data: Vec<FineTuneData>,
}

#[derive(Debug, Deserialize)]
pub struct FineTuneData {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: u64,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: HyperParams,
    pub organization_id: String,
    pub result_files: Vec<ResultFile>,
    pub status: String,
    pub validation_files: Vec<ValidationFile>,
    pub training_files: Vec<TrainingFile>,
    pub updated_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveFineTuneRequest {
    pub fine_tune_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveFineTuneResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub events: Vec<FineTuneEvent>,
    pub fine_tuned_model: Option<FineTunedModel>,
    pub hyperparams: HyperParams,
    pub organization_id: String,
    pub result_files: Vec<ResultFile>,
    pub status: String,
    pub validation_files: Vec<ValidationFile>,
    pub training_files: Vec<TrainingFile>,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CancelFineTuneRequest {
    pub fine_tune_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelFineTuneResponse {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: i64,
    pub events: Vec<FineTuneEvent>,
    pub fine_tuned_model: Option<String>,
    pub hyperparams: HyperParams,
    pub organization_id: String,
    pub result_files: Vec<ResultFile>,
    pub status: String,
    pub validation_files: Vec<ValidationFile>,
    pub training_files: Vec<TrainingFile>,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListFineTuneEventsRequest {
    pub fine_tune_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ListFineTuneEventsResponse {
    pub object: String,
    pub data: Vec<FineTuneEvent>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteFineTuneModelRequest {
    pub model_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteFineTuneModelResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
