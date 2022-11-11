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

#[derive(Serialize, Clone)]
pub struct Problem {
    pub id: i64,

    // A competition id, if the problem is in one.
    pub competition_id: Option<i64>,

    /// Problem title
    pub title: String,

    /// Problem description (markdown formatted)
    pub description: String,

    /// Code that parses standard input and outputs to standard out
    pub runner: String,

    /// Template that's shown when you start a problem
    pub template: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::problems))
        .route("/new", post(new::new))
        .route("/:problem_id", get(problem::problem))
        .route("/:problem_id/tests", get(tests::tests))
        .route("/:problem_id/history", get(history::history))
}
