use axum::{routing::get, Router};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use wasm_memory::AsymptoticComplexity;

mod invalidate;
mod new_completions;
mod submission;
mod tests;
mod validate;

#[derive(Deserialize, Serialize, PartialEq, Clone, FromRow)]
pub struct Submission {
    pub id: i64,
    pub problem_id: i64,
    pub user_id: i64,
    pub success: bool,
    pub runtime: i64,
    pub error: Option<String>,
    pub complexity: Option<AsymptoticComplexity>,
    pub time: NaiveDateTime,
    pub code: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/:submission_id", get(submission::submission))
        .route("/:submission_id/invalidate", get(invalidate::invalidate))
        .route("/:submission_id/validate", get(validate::validate))
        .route("/:submission_id/tests", get(tests::tests))
        .route("/new-completions", get(new_completions::new_completions))
}
