//! API endpoints relating to problems

use acm::models::{forms::CreateProblemForm, Auth};
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{Json, Path, ReqData},
    Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{api_error, api_success};
use crate::state::{auth::Claims, AppState};

/// Creates a new problem
///
/// **AUTHORIZATION**: ADMIN/OFFICER
#[post("/create-problem")]
pub async fn create_problem(
    form: Json<CreateProblemForm>,
    state: AppState,
    claims: ReqData<Claims>,
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

#[derive(Deserialize, Serialize)]
pub struct ProblemProps {
    id: u32,
}

/// Returns data associated with a given problem id
///
/// **AUTHORIZATION**: Any
#[get("/problems/{id}")]
pub async fn problem(id: Path<ProblemProps>, state: AppState) -> impl Responder {
    match state.problems_get_by_id(id.id).await {
        Some(problem) => api_success(problem),
        None => api_error(StatusCode::NOT_FOUND, "problem not found"),
    }
}