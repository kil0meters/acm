use axum::{Extension, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

use super::Activity;

#[derive(Deserialize)]
pub struct EditMeetingForm {
    pub title: String,
    pub description: String,
    pub meeting_time: NaiveDateTime,
    pub activities: Vec<Activity>,
}

#[derive(Serialize)]
pub struct EditMeetingBody {
    id: i64,
}

pub async fn edit(
    claims: Claims,
    Extension(pool): Extension<SqlitePool>,
    Json(form): Json<EditMeetingForm>,
) -> Result<Json<EditMeetingBody>, ServerError> {
    claims.validate_officer()?;

    let mut tx = pool.begin().await.map_err(|_| ServerError::InternalError)?;

    let id = sqlx::query!(
        r#"
        INSERT INTO meetings (
            title,
            description,
            meeting_time
        )
        VALUES (?, ?, ?)
        RETURNING id
        "#,
        form.title,
        form.description,
        form.meeting_time
    )
    .fetch_one(&mut tx)
    .await
    .map_err(|_| ServerError::InternalError)?
    .id;

    for activity in form.activities {
        sqlx::query!(
            r#"
            INSERT INTO activities (
                meeting_id,
                title,
                description,
                activity_type
            )
            VALUES (?, ?, ?, ?)
            "#,
            id,
            activity.title,
            activity.description,
            activity.activity_type
        )
        .execute(&mut tx)
        .await
        .map_err(|_| ServerError::InternalError)?;
    }

    tx.commit().await.map_err(|_| ServerError::InternalError)?;

    Ok(Json(EditMeetingBody { id }))
}
