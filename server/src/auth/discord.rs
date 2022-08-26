use std::{collections::HashMap, env};

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
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
    code: String,
    redirect_uri: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
}

#[derive(Deserialize)]
struct DiscordUser {
    username: String,
    discriminator: String,
    id: String
}

pub async fn login(
    Json(LoginForm { code, redirect_uri }): Json<LoginForm>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<LoginBody>, ServerError> {

    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("client_secret", env::var("DISCORD_SECRET").unwrap());
    params.insert("client_id", "984742374112624690".to_string());
    params.insert("grant_type", "authorization_code".to_string());
    params.insert("code", code);
    params.insert("redirect_uri", redirect_uri);


    let TokenResponse { access_token, token_type, expires_in } = client.post("https://discord.com/api/oauth2/token")
        .form(&params)
        .send()
        .await
        .map_err(|_| AuthError::InvalidToken)?
        .json()
        .await
        .map_err(|e| {
            log::error!("{}", e);
            ServerError::InternalError
        })?;

    let discord_user: DiscordUser = client.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("{token_type} {access_token}"))
        .send()
        .await
        .map_err(|e| {
            log::error!("{e}");
            AuthError::InvalidToken
        })?
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
        token,
    }))
}
