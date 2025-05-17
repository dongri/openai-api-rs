use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}
