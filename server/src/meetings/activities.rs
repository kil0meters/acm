use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::{Activity, ActivityType};

pub async fn activities(
    Path(meeting_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Activity>>, ServerError> {
    let activities = sqlx::query_as!(
        Activity,
        r#"
        SELECT
            id,
            title,
            description,
            activity_type as "activity_type: ActivityType"
        FROM
            activities
        WHERE
            meeting_id = ?
        "#,
        meeting_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    Ok(Json(activities))
}
