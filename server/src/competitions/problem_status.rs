use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use sqlx::{SqlitePool};

use crate::{auth::Claims, error::ServerError};

#[derive(Serialize)]
pub enum TeamStatus {
    Complete,   // At least one successful submission from any of the team members
    InProgress, // At least one submission from any of the team members
    NotStarted, // No submission from any of the team members
}

pub async fn problem_status(
    Path((competition_id, problem_id)): Path<(i64, i64)>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<TeamStatus>, ServerError> {
    claims.validate_logged_in()?;

    let success: Option<bool> = sqlx::query_scalar(
        r#"
        SELECT MAX(success) AS success FROM submissions
        JOIN team_members ON submissions.user_id = team_members.user_id
        JOIN teams ON team_members.team_id = teams.id
        JOIN competitions ON teams.competition_id = competitions.id
        WHERE submissions.problem_id = ?
        AND submissions.time > competitions.start
        AND submissions.time < competitions.end
        AND teams.id = (
            SELECT team_id FROM team_members
            JOIN teams ON team_members.team_id = teams.id
            WHERE teams.competition_id = ?
            AND team_members.user_id = ?
        )
    "#,
    )
    .bind(problem_id)
    .bind(competition_id)
    .bind(claims.user_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    Ok(Json(match success {
        Some(false) => TeamStatus::InProgress,
        Some(true) => TeamStatus::Complete,
        None => TeamStatus::NotStarted,
    }))
}
