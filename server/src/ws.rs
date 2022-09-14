use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    Extension,
};
use serde::Serialize;
use tokio::sync::broadcast::Sender;

use crate::{problems::Problem, submissions::Submission};

#[derive(Clone, Serialize)]
pub enum BroadcastMessage {
    // A user has submitted a valid solution to a problem
    NewCompletion(Submission),

    // A user is the first person to complete a problem
    NewStar(Submission),

    // New Problem
    NewProblem(Problem),
}

pub async fn handler(
    ws: WebSocketUpgrade,
    Extension(tx): Extension<Sender<BroadcastMessage>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: Sender<BroadcastMessage>) {
    let mut rx = tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let message =
            serde_json::to_string(&msg).expect("Could not convert message to JSON in websocket.");

        // send, or disconnect if the client disconnected
        if socket.send(Message::Text(message)).await.is_err() {
            return;
        }
    }
}
