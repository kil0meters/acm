use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::{
    auth::Auth,
    error::{ServerError, UserError},
};

#[derive(Serialize)]
pub struct UserBody {
    name: String,
    username: String,
    discord_id: String,
    auth: Auth,
}

pub async fn username(
    Path(username): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<UserBody>, ServerError> {
    let body = sqlx::query_as!(
        UserBody,
        r#"
        SELECT
            name,
            username,
            discord_id,
            auth as "auth: Auth"
        FROM
            users
        WHERE
            username = ?
        "#,
        username
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| UserError::NotFound(username))?;

    Ok(Json(body))
}


pub async fn id(
    Path(id): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<UserBody>, ServerError> {
    let body = sqlx::query_as!(
        UserBody,
        r#"
        SELECT
            name,
            username,
            discord_id,
            auth as "auth: Auth"
        FROM
            users
        WHERE
            id = ?
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| UserError::NotFound(id))?;

    Ok(Json(body))
}
