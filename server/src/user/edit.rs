use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    auth::{Auth, Claims},
    error::{ServerError, AuthError},
};

#[derive(Deserialize)]
pub struct EditUserForm {
    new_username: String,
    new_name: String,
    new_auth: Auth
}

pub async fn edit(
    Json(EditUserForm { new_username, new_name, mut new_auth }): Json<EditUserForm>,
    Path(username): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<()>, ServerError> {
    if claims.auth != Auth::ADMIN {
        if claims.username != username {
            return Err(AuthError::Unauthorized.into());
        } else {
            new_auth = claims.auth;
        }
    }

    sqlx::query!(r#"
        UPDATE users
        SET
            username = ?,
            name = ?,
            auth = ?
        WHERE
            username = ?
    "#,
        new_username,
        new_name,
        new_auth,
        username
    ).execute(&pool).await.map_err(|e| {
        log::error!("{e}");
        ServerError::InternalError
    })?;

    Ok(Json(()))
}
