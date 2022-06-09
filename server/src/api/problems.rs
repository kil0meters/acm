//! API endpoints relating to problems

use acm::models::{forms::CreateProblemForm, Auth};
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{Json, Path},
    HttpResponse, Responder,
};
use serde_json::json;

use super::{api_error, api_success};
use crate::{
    state::{auth::Claims, AppState},
    MAX_TEST_LENGTH,
};

/// Creates a new problem
///
/// **AUTHORIZATION**: ADMIN/OFFICER
#[post("/problems/new")]
pub async fn create_problem(
    form: Json<CreateProblemForm>,
    state: AppState,
    claims: Claims,
) -> impl Responder {
    match claims.auth {
        Auth::ADMIN | Auth::OFFICER => match state.problem_add(&form).await {
            Ok(id) => api_success(json!({ "id": id })),
            Err(_) => api_error(
                StatusCode::BAD_REQUEST,
                "A problem with that title already exists",
            ),
        },
        Auth::MEMBER => api_error(
            StatusCode::UNAUTHORIZED,
            "You must be an officer to do that",
        ),
    }
}

/// Shows all currently visisble problems
///
/// TODO: If the user is an officer/admin, it should show _ALL_ problms, regardless of whether they
/// are visible to the public or not.
///
/// **AUTHORIZATION**: Any
#[get("/problems")]
pub async fn problem_list(state: AppState) -> impl Responder {
    Json(state.problems_get().await)
}

/// Returns data associated with a given problem id
///
/// **AUTHORIZATION**: Any
#[get("/problems/{id}")]
pub async fn problem(id: Path<i64>, state: AppState) -> impl Responder {
    match state.problems_get_by_id(*id).await {
        Some(problem) => api_success(problem),
        None => api_error(StatusCode::NOT_FOUND, "problem not found"),
    }
}

/// Returns the tests for a specific problem id
///
/// **AUTHORIZATION**: Any
#[get("/problems/{id}/tests")]
pub async fn problem_tests(id: Path<i64>, state: AppState) -> impl Responder {
    let mut tests = state.tests_get_for_problem_id(*id).await;

    tests
        .iter_mut()
        .for_each(|test| test.truncate(MAX_TEST_LENGTH));

    Json(tests)
}

/// Gets the
///
/// **AUTHORIZATION**: Member
#[get("/problems/{problem_id}/history")]
pub async fn problem_history(
    problem_id: Path<i64>,
    state: AppState,
    claims: Claims,
) -> HttpResponse {
    let user_id = match state.get_user_id(&claims.username).await {
        Ok(id) => id,
        Err(_) => {
            return api_error(
                StatusCode::UNAUTHORIZED,
                "No user with your username exists",
            )
        }
    };

    api_success(state.problem_history(*problem_id, user_id).await)
}
