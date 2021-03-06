use axum::{
    routing::{get, post},
    Router,
};
use serde::Serialize;

mod history;
mod index;
mod new;
mod problem;
mod tests;

#[derive(Serialize)]
pub struct Problem {
    id: i64,

    /// Problem title
    title: String,

    /// Problem description (markdown formatted)
    description: String,

    /// Code that parses standard input and outputs to standard out
    runner: String,

    /// Template that's shown when you start a problem
    template: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::problems))
        .route("/new", post(new::new))
        .route("/:problem_id", get(problem::problem))
        .route("/:problem_id/tests", get(tests::tests))
        .route("/:problem_id/history", get(history::history))
}
