use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_builder_methods;

#[derive(Debug, Serialize, Clone)]
pub struct CreateFineTuningJobRequest {
    pub model: String,
    pub training_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperparameters: Option<HyperParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
}

impl CreateFineTuningJobRequest {
    pub fn new(model: String, training_file: String) -> Self {
        Self {
            model,
            training_file,
            hyperparameters: None,
            suffix: None,
            validation_file: None,
        }
    }
}

impl_builder_methods!(
    CreateFineTuningJobRequest,
    hyperparameters: HyperParameters,
    suffix: String,
    validation_file: String
);

#[derive(Debug, Serialize)]
pub struct ListFineTuningJobsRequest {
    // TODO pass as query params
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListFineTuningJobsRequest {
    pub fn new(_fine_tune_id: String) -> Self {
        Self {
            after: None,
            limit: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListFineTuningJobEventsRequest {
    pub fine_tuning_job_id: String,
    // TODO pass as query params
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListFineTuningJobEventsRequest {
    pub fn new(fine_tuning_job_id: String) -> Self {
        Self {
            fine_tuning_job_id,
            after: None,
            limit: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RetrieveFineTuningJobRequest {
    pub fine_tuning_job_id: String,
}

impl RetrieveFineTuningJobRequest {
    pub fn new(fine_tuning_job_id: String) -> Self {
        Self { fine_tuning_job_id }
    }
}

#[derive(Debug, Serialize)]
pub struct CancelFineTuningJobRequest {
    pub fine_tuning_job_id: String,
}

impl CancelFineTuningJobRequest {
    pub fn new(fine_tuning_job_id: String) -> Self {
        Self { fine_tuning_job_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningPagination<T> {
    pub object: String,
    pub data: Vec<T>,
    pub has_more: bool,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningJobObject {
    pub id: String,
    pub created_at: i64,
    pub error: Option<FineTuningJobError>,
    pub fine_tuned_model: Option<String>,
    pub finished_at: Option<String>,
    pub hyperparameters: HyperParameters,
    pub model: String,
    pub object: String,
    pub organization_id: String,
    pub result_files: Vec<String>,
    pub status: String,
    pub trained_tokens: Option<i64>,
    pub training_file: String,
    pub validation_file: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningJobError {
    pub code: String,
    pub message: String,
    pub param: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FineTuningJobEvent {
    pub id: String,
    pub created_at: i64,
    pub level: String,
    pub message: String,
    pub object: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HyperParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate_multiplier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_epochs: Option<String>,
}
