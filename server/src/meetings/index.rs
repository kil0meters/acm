use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Meeting;

pub async fn meetings(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Meeting>>, ServerError> {
    let meetings = sqlx::query_as!(
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
            meeting_time ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    Ok(Json(meetings))
}
