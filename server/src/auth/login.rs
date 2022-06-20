use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    auth::verify_password,
    error::{AuthError, ServerError},
};

use super::{Auth, Claims, User, KEYS};

#[derive(Serialize)]
pub struct LoginBody {
    user: User,
    token: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn login(
    Json(LoginForm { username, password }): Json<LoginForm>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<LoginBody>, ServerError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            name,
            username,
            password,
            auth as "auth: Auth"
        FROM
            users
        WHERE username = ?
        "#,
        username
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| AuthError::InvalidUsername)?;

    if verify_password(&username, &password, &user) {
        let claims = Claims {
            username: user.username.clone(),
            exp: usize::MAX,
            auth: user.auth,
        };

        let token = KEYS.encode_token(claims)?;

        Ok(Json(LoginBody { user, token }))
    } else {
        Err(AuthError::InvalidPassword.into())
    }
}
