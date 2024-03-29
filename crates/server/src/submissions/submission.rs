use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Submission;

pub async fn submission(
    Path(submission_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Submission>, ServerError> {
    let submission: Submission = sqlx::query_as(r#"SELECT * FROM submissions WHERE id = ?"#)
        .bind(submission_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| ServerError::NotFound)?;

    Ok(Json(submission))
}
