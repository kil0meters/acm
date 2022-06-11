use acm::models::{test::TestResult, Submission, User, forms::FirstTimeCompletionsForm};
use actix_web::{
    get,
    http::StatusCode,
    web::{Json, Path, Query},
    Responder,
};

use super::{api_error, api_success};
use crate::{state::AppState, MAX_TEST_LENGTH};

/// **AUTHORIZATION**: Any
#[get("/submissions/{submission_id}/tests")]
pub async fn submission_tests(submission_id: Path<i64>, state: AppState) -> Json<Vec<TestResult>> {
    let mut tests = state.tests_for_submission(*submission_id).await;

    tests.iter_mut().for_each(|test| {
        test.truncate(MAX_TEST_LENGTH);
    });

    Json(tests)
}

/// **AUTHORIZATION**: Any
#[get("/submissions/{submission_id}")]
pub async fn submission(submission_id: Path<i64>, state: AppState) -> impl Responder {
    match state.get_submission(*submission_id).await {
        Ok(sub) => api_success(sub),
        Err(_) => api_error(StatusCode::NOT_FOUND, "Submission not found"),
    }
}

#[get("/submissions/new-completions")]
pub async fn first_time_completions(state: AppState, form: Query<FirstTimeCompletionsForm>) -> Json<Vec<(User, Submission)>> {
    Json(state.first_completions(form.since).await)
}
