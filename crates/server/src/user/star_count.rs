use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::error::{ServerError, UserError};

#[derive(Serialize)]
pub struct StarCount {
    count: i32,
}

pub async fn star_count(
    Path(id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<StarCount>, ServerError> {
    let body = sqlx::query_as!(
        StarCount,
        r#"
        SELECT COUNT(DISTINCT(problem_id)) AS count
        FROM users
        JOIN submissions ON users.id = submissions.user_id
        WHERE success = true
        AND users.id = ?;
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| UserError::NotFound(id))?;

    Ok(Json(body))
}
