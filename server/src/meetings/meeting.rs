use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Meeting;

pub async fn meeting(
    Path(meeting_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Meeting>, ServerError> {
    let meeting = sqlx::query_as!(
        Meeting,
        r#"
        SELECT
            id,
            title,
            description,
            meeting_time
        FROM
            meetings
        WHERE
            id = ?
        "#,
        meeting_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(meeting))
}
