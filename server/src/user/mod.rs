use axum::{routing::{get, post}, Router};

mod submissions;
mod user_info;
mod edit;

pub fn routes() -> Router {
    Router::new()
        .route("/username/:username/submissions", get(submissions::submissions))
        .route("/username/:username", get(user_info::username))
        .route("/id/:id", get(user_info::id))
        .route("/edit/:user_id", post(edit::edit))
}
