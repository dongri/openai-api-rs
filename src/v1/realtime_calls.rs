use serde::{Deserialize, Serialize};

use crate::realtime::types::RealtimeModel;

/// Used to start a realtime session based on an incoming call that you can then connect to over WSS with `RealtimeSipClient` from `openai_api_rs::realtime::sip`.
/// Note that this is poorly documented by OpenAI with the only example data given in https://platform.openai.com/docs/guides/realtime-sip#handle-the-webhook and these may not be all the possible fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AcceptCallRequest {
    /// This is *always* `realtime`. Convenience constructor exposed to ensure this.
    #[serde(rename = "type")]
    pub session_type: String,
    pub instructions: String,
    pub model: RealtimeModel,
}
impl AcceptCallRequest {
    pub fn new(instructions: String, model: RealtimeModel) -> Self {
        Self {
            session_type: "realtime".to_string(),
            instructions,
            model,
        }
    }
}

/// Used to redirect a call to another number. Per https://platform.openai.com/docs/guides/realtime-sip#handle-the-webhook the Tel-URI scheme may be used.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReferCallRequest {
    /// The URI to redirect the call to, for example `tel:+14152909007`
    pub target_uri: String,
}
