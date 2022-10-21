use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::{
    auth::{Auth, Claims, User},
    error::{ServerError, UserError},
};

pub async fn me(
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<User>, ServerError> {
    claims.validate_logged_in()?;

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
        claims.user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| UserError::NotFound(format!("id {}", claims.user_id)))?;

    Ok(Json(body))
}
