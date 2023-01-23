use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Meeting;

pub async fn next(Extension(pool): Extension<SqlitePool>) -> Result<Json<Meeting>, ServerError> {
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
            DATETIME('now', 'localtime', 'start of day') < DATETIME(meeting_time)
        ORDER BY
            DATETIME(meeting_time) ASC
        "#
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(meeting))
}
