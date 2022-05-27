use acm::models::{Meeting, MeetingActivities};
use actix_web::{get, Responder, web::{Json, Path}};

use crate::state::AppState;

#[get("/meetings")]
pub async fn meeting_list(state: AppState) -> Json<Vec<Meeting>> {
    let meetings = state.get_future_meetings().await;

    log::warn!("{meetings:?}");

    Json(meetings)
}

#[get("/meetings/{id}")]
pub async fn meeting(id: Path<i64>, state: AppState) -> Json<Option<MeetingActivities>> {
    let id = *id;

    if let Some(meeting) = state.get_meeting(id).await {
        Json(Some(MeetingActivities {
            meeting,
            activities: state.get_activities_for_meeting(id).await
        }))
    } else {
        Json(None)
    }
}

#[get("/next-meeting")]
pub async fn next_meeting(state: AppState) -> Json<Option<MeetingActivities>> {
    if let Some(next_meeting) = state.get_next_meeting().await {
        Json(Some(MeetingActivities {
            activities: state.get_activities_for_meeting(next_meeting.id).await,
            meeting: next_meeting,
        }))
    } else {
        Json(None)
    }
}
