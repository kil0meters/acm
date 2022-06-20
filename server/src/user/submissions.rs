use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error::{ServerError, UserError};

#[derive(Deserialize)]
pub struct Pagination<const C: i64, const O: i64> {
    #[serde(default = "default_value::<O>")]
    count: i64,

    #[serde(default = "default_value::<C>")]
    offset: i64,
}

fn default_value<const T: i64>() -> i64 {
    T
}

#[derive(Serialize)]
pub struct SubmissionResult {
    id: i64,
    problem_id: i64,
    success: bool,
    runtime: i64,
    time: NaiveDateTime,
    error: Option<String>,
    code: String,
}

pub async fn submissions(
    Path(username): Path<String>,
    Query(pagination): Query<Pagination<0, 10>>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<SubmissionResult>>, ServerError> {
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
        SubmissionResult,
        r#"
        SELECT
            id,
            problem_id,
            success,
            runtime,
            time,
            error,
            code
        FROM
            submissions
        WHERE
            user_id = ?
        ORDER BY
            time DESC
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
