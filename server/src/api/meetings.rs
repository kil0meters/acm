use acm::models::Meeting;
use actix_web::{get, Responder, web::{Json, Path}};

use crate::state::AppState;

#[get("/meetings")]
pub async fn meeting_list(state: AppState) -> Json<Vec<Meeting>> {
    let meetings = state.get_future_meetings().await;

    log::warn!("{meetings:?}");

    Json(meetings)
}

#[get("/meetings/{id}")]
pub async fn meeting(id: Path<i64>, state: AppState) -> Json<Option<Meeting>> {
    Json(state.get_meeting(*id).await)
}

#[get("/next-meeting")]
pub async fn next_meeting(state: AppState) -> Json<Option<Meeting>> {
    Json(state.get_next_meeting().await)
}
