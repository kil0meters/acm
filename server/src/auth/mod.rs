// shamelessly ripped from:
// https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    routing::{get, post},
    Router,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::error::{AuthError, ServerError};

mod discord;

pub fn routes() -> Router {
    Router::new()
        .route("/discord", post(discord::login))
        .route("/logout", get(discord::logout))
}

#[derive(Serialize, Debug)]
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
pub enum Auth {
    ADMIN,
    OFFICER,
    MEMBER,
    LOGGED_OUT,
}

impl Default for Auth {
    fn default() -> Self {
        Auth::MEMBER
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
            Auth::ADMIN | Auth::OFFICER => Ok(()),
            Auth::MEMBER | Auth::LOGGED_OUT => Err(AuthError::WrongCredentials.into()),
        }
    }

    pub fn validate_logged_in(&self) -> Result<(), ServerError> {
        match self.auth {
            Auth::LOGGED_OUT => Err(AuthError::WrongCredentials.into()),
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let jar = CookieJar::from_request(req).await.unwrap();

        let token = match jar.get("token") {
            Some(cookie) => cookie.value(),
            None => {
                return Ok(Claims {
                    user_id: -1,
                    auth: Auth::LOGGED_OUT,
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
