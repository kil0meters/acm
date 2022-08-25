use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use serde_json::Value;

use crate::{
    error::{AuthError, ServerError},
};

use super::{Auth, Claims, User, KEYS};


#[derive(Serialize)]
pub struct LoginBody {
    user: User,
    discord_token: String,
    token: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    token_type: String,
    access_token: String,
    expires_in: usize,
}

#[derive(Deserialize)]
struct DiscordUser {
    username: String,
    discriminator: String,
    id: String
}

pub async fn login(
    Json(LoginForm { token_type, access_token, expires_in }): Json<LoginForm>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<LoginBody>, ServerError> {

    let client = reqwest::Client::new();

    let discord_user: DiscordUser = client.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("{token_type} {access_token}"))
        .send()
        .await
        .map_err(|_| AuthError::InvalidToken)?
        .json()
        .await
        .map_err(|_| ServerError::InternalError)?;

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            name,
            username,
            discord_id,
            auth as "auth: Auth"
        FROM
            users
        WHERE discord_id = ?
        "#,
        discord_user.id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    let user = match user {
        Some(user) => user,
        // If the user does not exist
        None => {
            // Try with base username, if that fails, include the descriminator.
            let user = sqlx::query_as!(
                User,
                r#"
                INSERT INTO users (
                    name,
                    username,
                    discord_id
                )
                VALUES (?, ?, ?)
                RETURNING
                    name,
                    username,
                    discord_id,
                    auth as "auth: Auth"
                "#,
                discord_user.username,
                discord_user.username,
                discord_user.id
            ).fetch_one(&pool)
            .await;

            match user {
                Ok(user) => user,
                Err(_) => {
                    let username = format!("{}#{}", discord_user.username, discord_user.discriminator);

                    let user = sqlx::query_as!(
                        User,
                        r#"
                        INSERT INTO users (
                            name,
                            username,
                            discord_id
                        )
                        VALUES (?, ?, ?)
                        RETURNING
                            name,
                            username,
                            discord_id,
                            auth as "auth: Auth"
                        "#,
                        discord_user.username,
                        username,
                        discord_user.id
                    ).fetch_one(&pool)
                    .await
                    .expect("Should not happen");

                    user
                },
            }
        },
    };

    let claims = Claims {
        username: user.username.clone(),
        // TODO: Copy this from whatever discord gives
        exp: expires_in,
        auth: user.auth,
    };

    let token = KEYS.encode_token(claims)?;

    Ok(Json(LoginBody {
        user,
        discord_token: access_token,
        token,
    }))
}
