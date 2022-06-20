use axum::{Extension, Json};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error::{FormValidationError, ServerError};

use super::{hash_password, Auth, Claims, User, KEYS};
use validator::Validate;

static RE_PASSWORD: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-zA-Z0-9!@#$%^&*()\s]{8,256})$").unwrap());

static RE_USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([a-zA-Z0-9]{1,16})$").unwrap());

#[derive(Serialize)]
pub struct SignupBody {
    user: User,
    token: String,
}

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    name: String,

    #[validate(regex(path = "RE_USERNAME"))]
    username: String,

    #[validate(regex(path = "RE_PASSWORD"))]
    password: String,
}

pub async fn signup(
    Json(form): Json<SignupForm>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<SignupBody>, ServerError> {
    form.validate()?;

    let password = hash_password(&form.username, &form.password);

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (
            name,
            auth,
            username,
            password
        ) VALUES (?, ?, ?, ?)
        RETURNING
            name,
            username,
            password,
            auth as "auth: Auth"
        "#,
        form.name,
        Auth::MEMBER,
        form.username,
        password
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| FormValidationError::InvalidField("username".into()))?;

    let claims = Claims {
        username: user.username.clone(),
        exp: usize::MAX,
        auth: user.auth,
    };

    tracing::info!("New signup: \"{}\"", form.username);

    let token = KEYS.encode_token(claims)?;

    Ok(Json(SignupBody { user, token }))
}
