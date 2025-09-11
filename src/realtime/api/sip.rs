use super::*;

/// Intended for connecting to an already existing Realtime session spawned by accepting an incoming SIP call from e.g. Twilio.
pub struct RealtimeSipClient {
    pub wss_url: String,
    pub api_key: String,
    pub call_id: String,
}

impl RealtimeSipClient {
    pub fn new(api_key: String, call_id: String) -> Self {
        let wss_url = std::env::var("WSS_URL").unwrap_or_else(|_| WSS_URL.to_owned());
        Self::new_with_endpoint(wss_url, api_key, call_id)
    }

    pub fn new_with_endpoint(wss_url: String, api_key: String, call_id: String) -> Self {
        Self {
            wss_url,
            api_key,
            call_id,
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
        let url = format!("{}?callId={}", self.wss_url, self.call_id);
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
