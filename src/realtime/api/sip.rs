use serde::{Deserialize, Serialize};

use crate::realtime::types::RealtimeModel;

use super::*;

/// Intended for connecting to an already existing Realtime session spawned by accepting an incoming SIP call from e.g. Twilio.
pub struct RealtimeSipClient {
    pub wss_url: String,
    pub api_key: String,
    pub call_id: String,
    pub model: RealtimeModel, // contrary to the OpenAI tutorial, joining an SIP session without a `model` param causes an "invalid_request_error.missing_model"
}

impl RealtimeSipClient {
    pub fn new(api_key: String, call_id: String, model: RealtimeModel) -> Self {
        let wss_url = std::env::var("WSS_URL").unwrap_or_else(|_| WSS_URL.to_owned());
        Self::new_with_endpoint(wss_url, api_key, call_id, model)
    }

    pub fn new_with_endpoint(
        wss_url: String,
        api_key: String,
        call_id: String,
        model: RealtimeModel,
    ) -> Self {
        Self {
            wss_url,
            api_key,
            call_id,
            model,
        }
    }

    pub async fn connect(
        &self,
    ) -> Result<
        (
            SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
            SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        ),
        Box<dyn std::error::Error>,
    > {
        let model_slug = serde_json::to_string(&self.model).unwrap();
        let model_slug = model_slug
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap();

        let url = format!("{}?call_id={}", self.wss_url, self.call_id);
        // let url = format!(
        //     "{}?call_id={}&model={}",
        //     self.wss_url, self.call_id, model_slug
        // );
        let mut request = url.into_client_request()?;
        let api_key = self.api_key.clone();
        request
            .headers_mut()
            .insert("Authorization", format!("Bearer {api_key}").parse()?);
        let (ws_stream, _) = connect_async(request).await?;
        let (write, read) = ws_stream.split();
        Ok((write, read))
    }
}

/// This is the payload of a `realtime.call.incoming` event webhook which is what OpenAI sends to your application when a call hits the SIP endpoint for your project.
/// Exposes some convenience methods for when a call comes from Twilio which is one of the more common use cases. `openai_call_id()` is what you will need to use accept/hangup endpoints.
///
/// # Example
/// ```rust
/// const INSTRUCTIONS: &str = "You are a helpful assistant.";
/// #[axum::debug_handler]
/// async fn call_webhook(
///     State(mut state): State<AppState>,
///     Json(event): Json<RealtimeCallIncoming>,
/// ) -> impl IntoResponse {
///     let number = event.caller_number();
///     let call_id = event.openai_call_id();
///     let twilio_sid = event.twilio_call_sid();
///     let account_sid = event.twilio_account_sid();
///     log::info!(
///         "Call coming in from {:?} with OpenAi ID {:?}, Twilio SID {:?} / account SID {:?}",
///         number,
///         call_id,
///         twilio_sid,
///         account_sid
///     );
///
///     let accept_call = AcceptCallRequest::new(INSTRUCTIONS, RealtimeModel::GptRealtime);
///
///     match state.openai_client.accept_call(call_id, accept_call).await {
///         Ok(_) => {
///             log::info!("Accepted call {}", call_id);
///         }
///         Err(err) => {
///             log::error!("Failed to accept call {}: {}", call_id, err);
///         }
///     };
///     ()
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeCallIncoming {
    pub id: String,
    /// Always `event`.
    pub object: String,
    pub created_at: i64,
    /// This should always be `realtime.call.incoming`.
    #[serde(rename = "type")]
    pub event_type: String,
    /// Contains the actual unique data per call. Look for `call_id` here or call `openai_call_id()`.
    pub data: RealTimeCallIncomingData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeCallIncomingData {
    pub call_id: String,
    pub sip_headers: Vec<SipHeader>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SipHeader {
    pub name: String,
    pub value: String,
}

impl RealtimeCallIncoming {
    /// Get the call ID from the event data
    pub fn openai_call_id(&self) -> &str {
        &self.data.call_id
    }

    /// Extract the caller's phone number from the "From" SIP header
    pub fn caller_number(&self) -> Option<String> {
        self.data
            .sip_headers
            .iter()
            .find(|header| header.name == "From")
            .and_then(|header| {
                // Parse the From header to extract the phone number
                // Format: "+48123123123" <sip:+48123123123@pstn.twilio.com:5060>;tag=...
                if let Some(start) = header.value.find('"') {
                    if let Some(end) = header.value[start + 1..].find('"') {
                        return Some(header.value[start + 1..start + 1 + end].to_string());
                    }
                }
                None
            })
    }

    /// Get the Twilio Call SID from the X-Twilio-CallSid SIP header
    pub fn twilio_call_sid(&self) -> Option<&str> {
        self.data
            .sip_headers
            .iter()
            .find(|header| header.name == "X-Twilio-CallSid")
            .map(|header| header.value.as_str())
    }

    /// Get the Twilio Account SID from the X-Twilio-AccountSid SIP header
    pub fn twilio_account_sid(&self) -> Option<&str> {
        self.data
            .sip_headers
            .iter()
            .find(|header| header.name == "X-Twilio-AccountSid")
            .map(|header| header.value.as_str())
    }

    /// Get a specific SIP header value by name
    pub fn get_sip_header(&self, name: &str) -> Option<&str> {
        self.data
            .sip_headers
            .iter()
            .find(|header| header.name == name)
            .map(|header| header.value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_twilio_event() {
        let json = r#"{"id": "evt_68bc6828707881908be189456b84cc07", "object": "event", "created_at": 1757177896, "type": "realtime.call.incoming", "data": {"call_id": "rtc_c5b6f97fe96f4c809b78916a9ac15748", "sip_headers": [{"name": "From", "value": "\"+48123123123\" <sip:+48123123123@pstn.twilio.com:5060>;tag=82568196_c3356d0b_03f1232a-01cf-4a4a-af25-bac077219d08"}, {"name": "X-Twilio-CallSid", "value": "CA080dd4bebc0320639d7ae33b82e80481"}, {"name": "X-Twilio-AccountSid", "value": "fake_data"}]}}"#;

        let event: RealtimeCallIncoming = serde_json::from_str(json).unwrap();

        assert_eq!(
            event.openai_call_id(),
            "rtc_c5b6f97fe96f4c809b78916a9ac15748"
        );
        assert_eq!(event.caller_number(), Some("+48123123123".to_string()));
        assert_eq!(
            event.twilio_call_sid(),
            Some("CA080dd4bebc0320639d7ae33b82e80481")
        );
        assert_eq!(event.twilio_account_sid(), Some("fake_data"));
    }
}
