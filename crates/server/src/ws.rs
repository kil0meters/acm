use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    Extension,
};

use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::sync::broadcast::Sender;

use crate::{
    auth::Claims, error::ServerError, problems::Problem, run::JobStatus, submissions::Submission,
};

#[derive(Clone, Serialize)]
pub enum BroadcastMessage {
    // A user has submitted a valid solution to a problem
    NewCompletion(Submission),

    // A user is the first person to complete a problem
    NewStar(Submission),

    // New Problem
    NewProblem(Problem),

    // New Job
    NewJob(JobStatus),

    FinishedJob(JobStatus),
    // New Team Submission
}

pub async fn handler(
    ws: WebSocketUpgrade,
    claims: Claims,
    Extension(tx): Extension<Sender<BroadcastMessage>>,
) -> Result<Response, ServerError> {
    claims.validate_officer()?;

    Ok(ws.on_upgrade(|socket| handle_socket(socket, tx)))
}

async fn handle_socket(socket: WebSocket, tx: Sender<BroadcastMessage>) {
    let mut rx = tx.subscribe();

    let (mut sender, mut receiver) = socket.split();

    let mut recv_task =
        tokio::spawn(async move { while let Some(Ok(_msg)) = receiver.next().await {} });

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let message = serde_json::to_string(&msg)
                .expect("Could not convert message to JSON in websocket.");

            log::info!("Sending message: {}", message);

            match sender.send(Message::Text(message)).await {
                Err(e) => {
                    log::info!("{e}");
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
