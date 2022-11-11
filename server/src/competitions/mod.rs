use axum::{
    routing::{get, post},
    Router,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::error::ServerError;

mod competition;
mod index;
mod leaderboard;
mod new;
mod problem_status;
mod teams;

#[derive(Serialize)]
pub struct Competition {
    id: i64,
    name: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

// verifies that a competition is editable: must be
async fn verify_time_competition(id: i64, pool: &SqlitePool) -> Result<bool, ServerError> {
    sqlx::query_scalar(
        "SELECT datetime('now') < end FROM competitions WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| ServerError::NotFound)
}

async fn verify_time_team(id: i64, pool: &SqlitePool) -> Result<bool, ServerError> {
    sqlx::query_scalar(
        "SELECT datetime('now') < end FROM competitions WHERE id = (SELECT competition_id FROM teams WHERE team_id = ?)",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| ServerError::NotFound)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::competitions))
        .route("/:id", get(competition::competition))
        .route("/new", post(new::new))
        .route(
            "/:id/problem-status/:problem_id",
            get(problem_status::problem_status),
        )
        .route("/:id/leaderboard", get(leaderboard::leaderboard))
        .nest("/:id/teams", teams::routes())
}
