use std::{collections::HashMap, env};

use axum::{Extension, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error::{AuthError, ServerError};

use super::{Auth, Claims, User, KEYS};

#[derive(Serialize)]
pub struct LoginBody {
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
}

#[derive(Deserialize)]
struct DiscordUser {
    username: String,
    discriminator: String,
    id: String,
}

async fn get_user(discord_id: &str, pool: &SqlitePool) -> Option<User> {
    let user = sqlx::query_as!(
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
        WHERE discord_id = ?
        "#,
        discord_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        log::warn!("Failed to fetch user: {e:?}");
    })
    .ok()
    .flatten();

    user
}

pub async fn login(
    Extension(pool): Extension<SqlitePool>,
    jar: CookieJar,
    Json(LoginForm { code, redirect_uri }): Json<LoginForm>,
) -> Result<CookieJar, ServerError> {
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("client_secret", env::var("DISCORD_SECRET").unwrap());
    params.insert("client_id", "984742374112624690".to_string());
    params.insert("grant_type", "authorization_code".to_string());
    params.insert("code", code);
    params.insert("redirect_uri", redirect_uri);

    let TokenResponse {
        access_token,
        token_type,
    } = client
        .post("https://discord.com/api/oauth2/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| {
            log::error!("{}", e);
            AuthError::InvalidToken
        })?
        .json()
        .await
        .map_err(|e| {
            log::error!("{}", e);
            ServerError::InternalError
        })?;

    let discord_user: DiscordUser = client
        .get("https://discord.com/api/users/@me")
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

    let user = get_user(&discord_user.id, &pool).await;

    let user = match user {
        Some(user) => user,
        // If the user does not exist
        None => {
            let sanitized_username: String = discord_user
                .username
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            // Try with base username, if that fails, include the descriminator.
            sqlx::query!(
                r#"
                INSERT INTO users (
                    name,
                    username,
                    discord_id
                )
                VALUES (?, ?, ?)
                "#,
                sanitized_username,
                sanitized_username,
                discord_user.id
            )
            .execute(&pool)
            .await
            .ok();

            let user = get_user(&discord_user.id, &pool).await;

            match user {
                Some(user) => user,
                None => {
                    let username = format!("{}_{}", sanitized_username, discord_user.discriminator);

                    log::info!("selected username: {username:?}");

                    sqlx::query!(
                        r#"
                        INSERT INTO users (
                            name,
                            username,
                            discord_id
                        )
                        VALUES (?, ?, ?)
                        "#,
                        sanitized_username,
                        username,
                        discord_user.id
                    )
                    .execute(&pool)
                    .await
                    .ok();

                    let user = get_user(&discord_user.id, &pool).await;

                    user.unwrap()
                }
            }
        }
    };

    let claims = Claims {
        user_id: user.id,
        exp: usize::MAX,
        auth: user.auth,
    };

    let token = KEYS.encode_token(claims)?;

    Ok(jar.add(
        Cookie::build("token", token)
            .http_only(true)
            .path("/")
            .permanent()
            .finish(),
    ))
}

pub async fn logout(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build("token", "").path("/").finish())
}
