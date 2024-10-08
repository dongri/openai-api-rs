use std::process::exit;
use std::env;

use futures_util::{future, pin_mut, StreamExt};
use openai_api_rs::realtime::api::RealtimeClient;
use openai_api_rs::realtime::client_event::{ConversationItemCreate, ResponseCreate};
use openai_api_rs::realtime::server_event::ServerEvent;
use openai_api_rs::realtime::types::Item;
use tokio::io::AsyncReadExt;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let model = "gpt-4o-realtime-preview-2024-10-01".to_string();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let realtime_client = RealtimeClient::new(api_key, model);

    let (write, read) = realtime_client.connect().await.unwrap();
    println!("WebSocket handshake complete");

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);

    let ws_to_stdout = {
        read.for_each(|message| async {
            let message = message.unwrap();
            match message {
                Message::Text(_) => {
                    let data = message.clone().into_data();
                    let server_event: ServerEvent = serde_json::from_slice(&data).unwrap();
                    match server_event {
                        ServerEvent::ResponseOutputItemDone(_event) => {
                            eprintln!();
                        }
                        ServerEvent::ResponseAudioTranscriptDelta(event) => {
                            eprint!("{}", event.delta.trim());
                        }
                        ServerEvent::Error(e) => {
                            eprint!("{e:?}");
                        }
                        _ => {}
                    }
                }
                Message::Close(_) => {
                    eprintln!("Close");
                    exit(0);
                }
                _ => {}
            }
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;

    Ok(())
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 2048];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        let text = String::from_utf8_lossy(&buf).into_owned();
        let item = Item::try_from(serde_json::json!({
            "type": "message",
            "role": "user",
            "content": [
                {
                    "type": "input_text",
                    "text": text.trim()
                }
            ]
        }))
        .unwrap();
        let event = ConversationItemCreate {
            item,
            ..Default::default()
        };
        let message: Message = event.into();
        tx.unbounded_send(message).unwrap();
        tx.unbounded_send(ResponseCreate::default().into()).unwrap();
    }
}
