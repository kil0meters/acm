use axum::{routing::get, Router};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

mod new_completions;
mod submission;
mod tests;

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct Submission {
    pub id: i64,
    pub problem_id: i64,
    pub user_id: i64,
    pub success: bool,
    pub runtime: i64,
    pub error: Option<String>,
    pub time: NaiveDateTime,
    pub code: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/:submission_id", get(submission::submission))
        .route("/:submission_id/tests", get(tests::tests))
        .route("/new-completions", get(new_completions::new_completions))
}
