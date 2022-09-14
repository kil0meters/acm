use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::{
    auth::{Auth, User},
    error::{ServerError, UserError},
};

pub async fn username(
    Path(username): Path<String>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<User>, ServerError> {
    let body = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
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
) -> Result<Json<User>, ServerError> {
    let body = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
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
