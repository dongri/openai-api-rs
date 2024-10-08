use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, protocol::Message},
    MaybeTlsStream, WebSocketStream,
};

const WSS_URL: &str = "wss://api.openai.com/v1/realtime";

pub struct RealtimeClient {
    pub wss_url: String,
    pub api_key: String,
    pub model: String,
}

impl RealtimeClient {
    pub fn new(api_key: String, model: String) -> Self {
        let wss_url = std::env::var("WSS_URL").unwrap_or_else(|_| WSS_URL.to_owned());
        Self::new_with_endpoint(wss_url, api_key, model)
    }

    pub fn new_with_endpoint(wss_url: String, api_key: String, model: String) -> Self {
        Self {
            wss_url,
            api_key,
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
        let url = format!("{}?model={}", self.wss_url, self.model);
        let mut request = url.into_client_request()?;
        let api_key = self.api_key.clone();
        request
            .headers_mut()
            .insert("Authorization", format!("Bearer {api_key}").parse()?);
        request
            .headers_mut()
            .insert("OpenAI-Beta", "realtime=v1".parse()?);
        let (ws_stream, _) = connect_async(request).await?;
        let (write, read) = ws_stream.split();
        Ok((write, read))
    }
}
