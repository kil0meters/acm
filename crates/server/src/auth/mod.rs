// shamelessly ripped from:
// https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs

use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::request::Parts,
    routing::{get, post},
    Router,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

use crate::error::{AuthError, ServerError};

mod discord;

pub fn routes() -> Router {
    Router::new()
        .route("/discord", post(discord::login))
        .route("/logout", get(discord::logout))
}

#[derive(Serialize, Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,

    pub discord_id: String,
    pub auth: Auth,
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap();
    Keys::new(secret.as_bytes())
});

struct Keys {
    decoding: DecodingKey,
    encoding: EncodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }

    fn encode_token(&self, claims: Claims) -> Result<String, ServerError> {
        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding)
            .map_err(|_| AuthError::TokenCreation.into())
    }
}

#[derive(Debug, Clone, Copy, Type, Deserialize, Serialize, PartialEq)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Auth {
    Admin,
    Officer,
    Member,
    LoggedOut,
}

impl Default for Auth {
    fn default() -> Self {
        Auth::Member
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub auth: Auth,
    pub exp: usize,
}

impl Claims {
    /// Raises an error if the request is not of sufficient authorization
    pub fn validate_officer(&self) -> Result<(), ServerError> {
        match self.auth {
            Auth::Admin | Auth::Officer => Ok(()),
            Auth::Member | Auth::LoggedOut => Err(AuthError::WrongCredentials.into()),
        }
    }

    pub fn validate_logged_in(&self) -> Result<(), ServerError> {
        match self.auth {
            Auth::LoggedOut => Err(AuthError::WrongCredentials.into()),
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let jar = CookieJar::from_request_parts(req, state).await.unwrap();

        let token = match jar.get("token") {
            Some(cookie) => cookie.value(),
            None => {
                return Ok(Claims {
                    user_id: -1,
                    auth: Auth::LoggedOut,
                    exp: 0,
                })
            }
        };

        // Decode the user data
        let token_data =
            jsonwebtoken::decode::<Claims>(token, &KEYS.decoding, &Validation::default()).map_err(
                |e| {
                    log::error!("{e}");
                    AuthError::InvalidToken
                },
            )?;

        Ok(token_data.claims)
    }
}
