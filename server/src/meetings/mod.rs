use axum::{routing::{get, post}, Router};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

mod activities;
mod edit;
mod index;
mod meeting;
mod next;

#[derive(Serialize)]
pub struct Meeting {
    id: i64,
    title: String,
    description: String,
    meeting_time: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    SOLO,
    PAIR,
    LECT,
}

impl Default for ActivityType {
    fn default() -> Self {
        ActivityType::SOLO
    }
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    #[serde(default = "zero")]
    pub id: i64,
    pub title: String,
    pub description: String,
    pub activity_type: ActivityType,
}

fn zero() -> i64 {
    0
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::meetings))
        .route("/:meeting_id", get(meeting::meeting))
        .route("/next", get(next::next))
        .route("/:meeting_id/activities", get(activities::activities))
        .route("/edit", post(edit::edit))
}
