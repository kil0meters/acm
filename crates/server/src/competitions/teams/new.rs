use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

#[derive(Debug, Deserialize)]
pub struct NewTeamForm {
    name: String,
}

#[derive(Serialize)]
pub struct NewTeamBody {
    id: i64,
}

pub async fn new(
    claims: Claims,
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Json(form): Json<NewTeamForm>,
) -> Result<Json<NewTeamBody>, ServerError> {
    claims.validate_officer()?;

    let id = sqlx::query!(
        r#"INSERT INTO teams (competition_id, name) VALUES (?, ?) RETURNING id"#,
        id,
        form.name,
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?
    .id;

    Ok(Json(NewTeamBody { id }))
}
