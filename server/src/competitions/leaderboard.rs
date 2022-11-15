use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

use crate::error::ServerError;

#[derive(Serialize, FromRow)]
pub struct TeamLeaderboardEntry {
    id: i64,
    name: String,
    score: i64,
}

pub async fn leaderboard(
    Path(competition_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<TeamLeaderboardEntry>>, ServerError> {
    // god save me
    let leaderboard: Vec<TeamLeaderboardEntry> = sqlx::query_as(
        r#"
        SELECT id, name, COALESCE(score,0) AS score FROM teams LEFT JOIN (
            SELECT team_id, COUNT(team_id) AS score, time FROM (
                SELECT team_members.team_id AS team_id, submissions.time AS time
                FROM (SELECT * FROM submissions WHERE submissions.success = true) AS submissions
                JOIN (SELECT * FROM problems WHERE problems.competition_id = $1) AS problems ON problems.id = submissions.problem_id
                JOIN team_members ON team_members.user_id = submissions.user_id
                JOIN (SELECT * FROM teams WHERE teams.competition_id = $1) AS teams ON teams.id = team_members.team_id
                JOIN competitions ON teams.competition_id = competitions.id
                WHERE competitions.start < submissions.time AND competitions.end > submissions.time
                GROUP BY submissions.problem_id, teams.id
            ) GROUP BY team_id
        ) AS leaderboard ON leaderboard.team_id = teams.id
        WHERE teams.competition_id = $1
        ORDER BY score DESC, time ASC"#)
        .bind(competition_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            log::error!("{e}");
            ServerError::InternalError
        })?;

    Ok(Json(leaderboard))
}
