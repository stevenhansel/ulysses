use std::sync::Arc;

use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};

use crate::context::Context;
use crate::error::AppError;

/// GET /api/proxy/ws
///
/// Upgrades the connection to a WebSocket. Echoes messages back
/// as a placeholder — will be replaced with real-time model streaming.
pub async fn ws_handler(
    State(context): State<Arc<Context>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, context))
}

async fn handle_ws(socket: WebSocket, _context: Arc<Context>) {
    let (sender, receiver) = socket.split();

    // Spawn a task to forward incoming messages (placeholder echo)
    let handle = tokio::spawn(echo_loop(sender, receiver));

    if let Err(e) = handle.await {
        tracing::error!("WebSocket handler error: {:?}", e);
    }
}

async fn echo_loop(
    mut sender: SplitSink<WebSocket, Message>,
    mut receiver: impl StreamExt<Item = Result<Message, axum::Error>> + Unpin,
) -> Result<(), AppError> {
    while let Some(msg) = receiver.next().await {
        let msg = msg.map_err(|e| AppError::WebSocket(e.to_string()))?;

        match msg {
            Message::Text(text) => {
                sender
                    .send(Message::Text(text))
                    .await
                    .map_err(|e| AppError::WebSocket(e.to_string()))?;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    Ok(())
}
