use super::responses::CreateResponseRequest;
use futures_util::Stream;
use serde_json::Value;
use std::pin::Pin;
use std::task::{Context, Poll};

pub type CreateResponseStreamRequest = CreateResponseRequest;

#[derive(Debug, Clone)]
pub struct ResponseStreamEvent {
    pub event: Option<String>,
    pub data: Value,
}

#[derive(Debug, Clone)]
pub enum ResponseStreamResponse {
    Event(ResponseStreamEvent),
    Done,
}

pub struct ResponseStream<S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin> {
    pub response: S,
    pub buffer: String,
    pub first_chunk: bool,
}

impl<S> ResponseStream<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
{
    fn find_event_delimiter(buffer: &str) -> Option<(usize, usize)> {
        let carriage_idx = buffer.find("\r\n\r\n");
        let newline_idx = buffer.find("\n\n");

        match (carriage_idx, newline_idx) {
            (Some(r_idx), Some(n_idx)) => {
                if r_idx <= n_idx {
                    Some((r_idx, 4))
                } else {
                    Some((n_idx, 2))
                }
            }
            (Some(r_idx), None) => Some((r_idx, 4)),
            (None, Some(n_idx)) => Some((n_idx, 2)),
            (None, None) => None,
        }
    }

    fn next_response_from_buffer(&mut self) -> Option<ResponseStreamResponse> {
        while let Some((idx, delimiter_len)) = Self::find_event_delimiter(&self.buffer) {
            let event_block = self.buffer[..idx].to_owned();
            self.buffer = self.buffer[idx + delimiter_len..].to_owned();

            let mut event_name = None;
            let mut data_payload = String::new();

            for line in event_block.lines() {
                let trimmed_line = line.trim_end_matches('\r');

                if let Some(event) = trimmed_line
                    .strip_prefix("event: ")
                    .or_else(|| trimmed_line.strip_prefix("event:"))
                {
                    let name = event.trim();
                    if !name.is_empty() {
                        event_name = Some(name.to_string());
                    }
                } else if let Some(content) = trimmed_line
                    .strip_prefix("data: ")
                    .or_else(|| trimmed_line.strip_prefix("data:"))
                {
                    if !content.is_empty() {
                        if !data_payload.is_empty() {
                            data_payload.push('\n');
                        }
                        data_payload.push_str(content);
                    }
                }
            }

            if data_payload.is_empty() {
                continue;
            }

            if data_payload.trim() == "[DONE]" {
                return Some(ResponseStreamResponse::Done);
            }

            let parsed = serde_json::from_str::<Value>(&data_payload)
                .unwrap_or_else(|_| Value::String(data_payload.clone()));

            return Some(ResponseStreamResponse::Event(ResponseStreamEvent {
                event: event_name,
                data: parsed,
            }));
        }

        None
    }
}

impl<S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin> Stream for ResponseStream<S> {
    type Item = ResponseStreamResponse;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(response) = self.next_response_from_buffer() {
                return Poll::Ready(Some(response));
            }

            match Pin::new(&mut self.as_mut().response).poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    let chunk_str = String::from_utf8_lossy(&chunk).to_string();
                    if self.first_chunk {
                        self.first_chunk = false;
                    }
                    self.buffer.push_str(&chunk_str);
                }
                Poll::Ready(Some(Err(error))) => {
                    eprintln!("Error in stream: {:?}", error);
                    return Poll::Ready(None);
                }
                Poll::Ready(None) => {
                    return Poll::Ready(None);
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}
