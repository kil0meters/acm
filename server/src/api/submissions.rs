use acm::models::test::TestResult;
use actix_web::{
    get,
    http::StatusCode,
    web::{Json, Path},
    Responder,
};

use super::{api_error, api_success};
use crate::state::AppState;

/// **AUTHORIZATION**: Any
#[get("/submissions/{submission_id}/tests")]
pub async fn submission_tests(submission_id: Path<i64>, state: AppState) -> Json<Vec<TestResult>> {
    Json(state.tests_for_submission(submission_id.into_inner()).await)
}

/// **AUTHORIZATION**: Any
#[get("/submissions/{submission_id}")]
pub async fn submission(submission_id: Path<i64>, state: AppState) -> impl Responder {
    match state.get_submission(*submission_id).await {
        Ok(sub) => api_success(sub),
        Err(_) => api_error(StatusCode::NOT_FOUND, "Submission not found"),
    }
}
