use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    Extension,
};
use futures::{StreamExt, SinkExt};
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

async fn handle_socket(socket: WebSocket, tx: Sender<BroadcastMessage>) {
    let mut rx = tx.subscribe();

    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(_text))) = receiver.next().await {}
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let message =
                serde_json::to_string(&msg).expect("Could not convert message to JSON in websocket.");

            if sender.send(Message::Text(message)).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

}
