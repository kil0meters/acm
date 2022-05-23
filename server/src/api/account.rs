use acm::models::Pagination;
use actix_web::{
    get,
    http::StatusCode,
    web::{Path, Query},
    Responder,
};

use super::{api_error, api_success};
use crate::state::AppState;

#[get("/user-info/{username}")]
pub async fn user_info(username: Path<String>, state: AppState) -> impl Responder {
    match state.user_query(&username).await {
        Ok(user) => api_success(user),
        Err(_) => api_error(StatusCode::NOT_FOUND, "User not found"),
    }
}

#[get("/user-info/{username}/submissions")]
pub async fn user_submissions(
    username: Path<String>,
    pagination: Query<Pagination>,
    state: AppState,
) -> impl Responder {
    let res = state
        .recent_submissions_for_user(
            &username,
            pagination.count.unwrap_or(10),
            pagination.offset.unwrap_or(0),
        )
        .await;

    match res {
        Ok(res) => api_success(res),
        Err(e) => api_error(StatusCode::NOT_FOUND, e.to_string()),
    }
}
