use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    auth::{Auth, Claims, User},
    error::{AuthError, ServerError},
};

#[derive(Deserialize)]
pub struct EditUserForm {
    new_username: String,
    new_name: String,
    new_auth: Auth,
}

pub async fn edit(
    Path(user_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
    Json(EditUserForm {
        new_username,
        new_name,
        mut new_auth,
    }): Json<EditUserForm>,
) -> Result<Json<User>, ServerError> {
    claims.validate_logged_in()?;

    if claims.auth != Auth::Admin {
        if claims.user_id != user_id {
            return Err(AuthError::Unauthorized.into());
        } else {
            new_auth = claims.auth;
        }
    }

    let new_user: User = sqlx::query_as(
        r#"
        UPDATE users
        SET
            username = ?,
            name = ?,
            auth = ?
        WHERE
            id = ?
        RETURNING
            id,
            username,
            name,
            auth as 'auth: Auth',
            discord_id;
    "#,
    )
    .bind(new_username)
    .bind(new_name)
    .bind(new_auth)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        log::error!("{e}");
        ServerError::InternalError
    })?;

    Ok(Json(new_user))
}
