use axum::{routing::get, Router};

mod submissions;
mod username;

pub fn routes() -> Router {
    Router::new()
        .route("/:username/submissions", get(submissions::submissions))
        .route("/:username", get(username::username))
}
