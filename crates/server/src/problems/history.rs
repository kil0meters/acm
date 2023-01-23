use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

use crate::{auth::Claims, error::ServerError, pagination::Pagination};

#[derive(Serialize, FromRow)]
pub struct HistoryItem {
    id: i64,
    success: bool,
    runtime: i64,
    error: Option<String>,
    time: NaiveDateTime,
}

pub async fn history(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    Query(pagination): Query<Pagination<0, 10>>,
    claims: Claims,
) -> Result<Json<Vec<HistoryItem>>, ServerError> {
    claims.validate_logged_in()?;

    let submissions: Vec<HistoryItem> = sqlx::query_as(
        r#"
        SELECT
            id,
            success,
            runtime,
            error,
            time
        FROM
            submissions
        WHERE
            user_id = ? and problem_id = ?
        ORDER BY
            time DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(claims.user_id)
    .bind(problem_id)
    .bind(pagination.count)
    .bind(pagination.offset)
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(submissions))
}
