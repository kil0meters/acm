use axum::{routing::get, Router};

mod submissions;
mod user_info;

pub fn routes() -> Router {
    Router::new()
        .route("/username/:username/submissions", get(submissions::submissions))
        .route("/username/:username", get(user_info::username))
        .route("/id/:id", get(user_info::id))
}
