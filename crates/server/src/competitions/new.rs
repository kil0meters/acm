use axum::{Extension, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

#[derive(Debug, Deserialize)]
pub struct Competition {
    name: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

#[derive(Serialize)]
pub struct NewCompletionBody {
    id: i64,
}

pub async fn new(
    claims: Claims,
    Extension(pool): Extension<SqlitePool>,
    Json(form): Json<Competition>,
) -> Result<Json<NewCompletionBody>, ServerError> {
    claims.validate_officer()?;

    let id = sqlx::query!(
        r#"INSERT INTO competitions (name, start, end) VALUES (?, ?, ?) RETURNING id"#,
        form.name,
        form.start,
        form.end
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?
    .id;

    Ok(Json(NewCompletionBody { id }))
}
