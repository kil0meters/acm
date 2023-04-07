use axum::{extract::Path, Extension, Json};

use serde::Deserialize;
use sqlx::SqlitePool;

use super::Difficulty;
use crate::{auth::Claims, error::ServerError};

#[derive(Deserialize)]
pub struct EditForm {
    title: String,
    description: String,
    template: String,
    visible: bool,
    difficulty: Difficulty,
}

pub async fn edit(
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
    Path(id): Path<i64>,
    Json(form): Json<EditForm>,
) -> Result<(), ServerError> {
    claims.validate_officer()?;

    log::info!("User {} editing problem {}", claims.user_id, id);

    sqlx::query!(
        r#"
        UPDATE problems SET
        title = ?,
        description = ?,
        difficulty = ?,
        visible = ?,
        template = ?
        WHERE id = ?
        "#,
        form.title,
        form.description,
        form.difficulty,
        form.visible,
        form.template,
        id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        log::error!("{e}");
        ServerError::InternalError
    })?;

    Ok(())
}
