use acm::models::{forms::EditMeetingForm, Auth, Meeting, MeetingActivities};
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{Json, Path},
    Responder,
};
use serde_json::json;

use crate::state::{auth::Claims, AppState};

use super::{api_error, api_success};

#[post("/meetings/edit")]
pub async fn edit_meeting(
    form: Json<EditMeetingForm>,
    state: AppState,
    claims: Claims,
) -> impl Responder {
    let form = form.into_inner();

    log::info!("Meeting edited by {}", claims.username);

    match claims.auth {
        Auth::ADMIN | Auth::OFFICER => match state.edit_or_insert_meeting(&form).await {
            Ok(id) => api_success(json!({ "id": id })),
            Err(_) => api_error(StatusCode::UNPROCESSABLE_ENTITY, "Bad input"),
        },
        Auth::MEMBER => api_error(
            StatusCode::UNAUTHORIZED,
            "You must be an officer to do that",
        ),
    }
}

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
            activities: state.get_activities_for_meeting(id).await,
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
