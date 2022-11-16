use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError, pagination::Pagination};

#[derive(Serialize)]
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

    let submissions = sqlx::query_as!(
        HistoryItem,
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
        claims.user_id,
        problem_id,
        pagination.count,
        pagination.offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(submissions))
}
