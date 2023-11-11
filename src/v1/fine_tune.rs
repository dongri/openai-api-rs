use serde::{Deserialize, Serialize};

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
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

impl CreateFineTuneRequest {
    pub fn new(training_file: String) -> Self {
        Self {
            training_file,
            validation_file: None,
            model: None,
            n_epochs: None,
            batch_size: None,
            learning_rate_multiplier: None,
            prompt_loss_weight: None,
            compute_classification_metrics: None,
            classification_n_classes: None,
            classification_positive_class: None,
            classification_betas: None,
            suffix: None,
        }
    }
}

impl_builder_methods!(
    CreateFineTuneRequest,
    validation_file: String,
    model: String,
    n_epochs: i32,
    batch_size: i32,
    learning_rate_multiplier: f32,
    prompt_loss_weight: f32,
    compute_classification_metrics: bool,
    classification_n_classes: i32,
    classification_positive_class: String,
    classification_betas: Vec<f32>,
    suffix: String
);

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

impl RetrieveFineTuneRequest {
    pub fn new(fine_tune_id: String) -> Self {
        Self { fine_tune_id }
    }
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

impl CancelFineTuneRequest {
    pub fn new(fine_tune_id: String) -> Self {
        Self { fine_tune_id }
    }
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

impl ListFineTuneEventsRequest {
    pub fn new(fine_tune_id: String) -> Self {
        Self { fine_tune_id }
    }
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

impl DeleteFineTuneModelRequest {
    pub fn new(model_id: String) -> Self {
        Self { model_id }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteFineTuneModelResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
