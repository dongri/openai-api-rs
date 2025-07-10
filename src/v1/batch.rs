use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBatchRequest {
    pub input_file_id: String,
    pub endpoint: String,
    pub completion_window: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub customer_id: String,
    pub batch_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCounts {
    pub total: u32,
    pub completed: u32,
    pub failed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchError {
    pub code: String,
    pub line: Option<u32>,
    pub message: String,
    pub param: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchErrors {
    pub object: String,
    pub data: Vec<BatchError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponse {
    pub cancelled_at: Option<u64>,
    pub cancelling_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub completion_window: String,
    pub created_at: u64,
    pub endpoint: String,
    pub error_file_id: Option<String>,
    pub errors: Option<BatchErrors>,
    pub expired_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub failed_at: Option<u64>,
    pub finalizing_at: Option<u64>,
    pub id: String,
    pub in_progress_at: Option<u64>,
    pub input_file_id: String,
    pub metadata: Option<Metadata>,
    pub object: String,
    pub output_file_id: Option<String>,
    pub request_counts: RequestCounts,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBatchResponse {
    pub object: String,
    pub data: Vec<BatchResponse>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

impl CreateBatchRequest {
    pub fn new(input_file_id: String, endpoint: String, completion_window: String) -> Self {
        Self {
            input_file_id,
            endpoint,
            completion_window,
            metadata: None,
        }
    }
}
