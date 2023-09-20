use axum::{extract::Path, Extension, Json};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

use crate::error::ServerError;

#[derive(Serialize, FromRow)]
pub struct ProblemLeaderboardItem {
    submission_id: i64,
    runtime: i64,
    name: String,
    username: String,
    time: NaiveDateTime,
}

pub async fn leaderboard_users(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<ProblemLeaderboardItem>>, ServerError> {
    // if the problem is in a competition that is still active, we simply return an empty list
    let res = sqlx::query_scalar!(
        "SELECT datetime('now') < end
        FROM competitions
        JOIN problems
        WHERE competitions.id = problems.competition_id
        AND problems.id = ?",
        problem_id
    )
    .fetch_one(&pool)
    .await;

    if let Ok(res) = res {
        if res == 1 {
            return Err(ServerError::NotFound);
        }
    }

    let items = sqlx::query_as(
        r#"SELECT
        submissions.id AS submission_id,
        MIN(submissions.runtime) AS runtime,
        users.name AS name,
        users.username AS username,
        submissions.time AS time
        FROM submissions
        JOIN users ON submissions.user_id = users.id
        WHERE problem_id = ?
        AND success = true
        GROUP BY submissions.user_id
        ORDER BY runtime, submissions.time"#,
    )
    .bind(problem_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(items))
}

pub async fn leaderboard_submissions(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<ProblemLeaderboardItem>>, ServerError> {
    // if the problem is in a competition that is still active, we simply return an empty list
    let res = sqlx::query_scalar!(
        "SELECT datetime('now') < end
        FROM competitions
        JOIN problems
        WHERE competitions.id = problems.competition_id
        AND problems.id = ?",
        problem_id
    )
    .fetch_one(&pool)
    .await;

    if let Ok(res) = res {
        if res == 1 {
            return Err(ServerError::NotFound);
        }
    }

    let items = sqlx::query_as(
        r#"SELECT
        submissions.id AS submission_id,
        submissions.runtime AS runtime,
        users.name AS name,
        users.username AS username,
        submissions.time AS time
        FROM submissions
        JOIN users ON submissions.user_id = users.id
        WHERE problem_id = ?
        AND success = true
        GROUP BY runtime
        ORDER BY runtime, submissions.time"#,
    )
    .bind(problem_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(items))
}

pub async fn leaderboard_slow_submissions(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<ProblemLeaderboardItem>>, ServerError> {
    // if the problem is in a competition that is still active, we simply return an empty list
    let res = sqlx::query_scalar!(
        "SELECT datetime('now') < end
        FROM competitions
        JOIN problems
        WHERE competitions.id = problems.competition_id
        AND problems.id = ?",
        problem_id
    )
    .fetch_one(&pool)
    .await;

    if let Ok(res) = res {
        if res == 1 {
            return Err(ServerError::NotFound);
        }
    }

    let items = sqlx::query_as(
        r#"SELECT
        submissions.id AS submission_id,
        submissions.runtime AS runtime,
        users.name AS name,
        users.username AS username,
        submissions.time AS time
        FROM submissions
        JOIN users ON submissions.user_id = users.id
        WHERE problem_id = ?
        AND success = true
        GROUP BY runtime
        ORDER BY runtime DESC, submissions.time"#,
    )
    .bind(problem_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(items))
}
