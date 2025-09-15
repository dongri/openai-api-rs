use serde::{Deserialize, Serialize};

use crate::realtime::types::Session;

/// Used to start a realtime session based on an incoming call that you can then connect to over WSS with `RealtimeSipClient` from `openai_api_rs::realtime::sip`.
/// Note that this is poorly documented by OpenAI with the only example data given in https://platform.openai.com/docs/guides/realtime-sip#handle-the-webhook and these may not be all the possible fields.
/// Per an OpenAI dev (https://community.openai.com/t/how-to-setup-transcription-on-realtime-api-with-sip/1355068/12) anything that can be passed to `session.update` over WSS can be passed to /accept,
/// as well as `model`, ordinarily reserved for `session.create`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AcceptCallRequest {
    /// The session must *always* be a `realtime` one.
    #[serde(flatten)]
    pub session: Session,
}

/// Used to redirect a call to another number. Per https://platform.openai.com/docs/guides/realtime-sip#handle-the-webhook the Tel-URI scheme may be used.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReferCallRequest {
    /// The URI to redirect the call to, for example `tel:+14152909007`
    pub target_uri: String,
}
