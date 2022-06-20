use axum::{routing::post, Router};

mod custom;
mod generate_tests;
mod submit;

pub fn routes() -> Router {
    Router::new()
        .route("/custom", post(custom::custom))
        .route("/generate-tests", post(generate_tests::generate_tests))
        .route("/submit", post(submit::submit))
}
