use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::{
    error::{ServerError, UserError},
    pagination::Pagination,
};

#[derive(Serialize)]
pub struct UserSubmission {
    id: i64,
    problem_title: String,
    success: bool,
    error: Option<String>,
    runtime: i64,
    code: String,
    time: NaiveDateTime,
}

pub async fn submissions(
    Path(username): Path<String>,
    Query(pagination): Query<Pagination<0, 10>>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<UserSubmission>>, ServerError> {
    let user_id = sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE username = ?
        "#,
        username
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| UserError::NotFound(username))?
    .id;

    let submissions = sqlx::query_as!(
        UserSubmission,
        r#"
        SELECT
            submissions.id,
            problems.title AS problem_title,
            submissions.success,
            submissions.runtime,
            submissions.error,
            submissions.time,
            submissions.code
        FROM submissions
        JOIN problems ON problems.id = submissions.problem_id
        LEFT JOIN competitions ON competitions.id = problems.competition_id
        WHERE user_id = ?
        AND (
            problems.competition_id IS NULL
            OR competitions.end < datetime('now')
        )
        ORDER BY time DESC
        LIMIT ? OFFSET ?
        "#,
        user_id,
        pagination.count,
        pagination.offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| UserError::InternalError)?;

    Ok(Json(submissions))
}
