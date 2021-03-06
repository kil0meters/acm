// shamelessly ripped from:
// https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    routing::post,
    Router,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::error::{AuthError, ServerError};

mod login;
mod signup;

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login::login))
        .route("/signup", post(signup::signup))
}

// Returns the hash of a password
fn hash_password(username: &str, password: &str) -> String {
    // TODO: This is probably not very secure, should use a random salt stored in the database
    // rather than the username for salt.
    let salted_pass = format!("{}{}", username, password);
    bcrypt::hash(salted_pass.as_bytes(), bcrypt::DEFAULT_COST).unwrap()
}

/// Verifies that a password matches a given hash
fn verify_password(username: &str, password: &str, user: &User) -> bool {
    let salted_pass = format!("{}{}", username, password);
    bcrypt::verify(&salted_pass, &user.password).unwrap()
}

#[derive(Serialize)]
struct User {
    name: String,
    username: String,

    #[serde(skip)]
    password: String,
    auth: Auth,
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
}

impl Default for Auth {
    fn default() -> Self {
        Auth::MEMBER
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub auth: Auth,
    pub exp: usize,
}

impl Claims {
    /// Raises an error if the request is not of sufficient authorization
    pub fn validate_officer(&self) -> Result<(), ServerError> {
        match self.auth {
            Auth::ADMIN | Auth::OFFICER => Ok(()),
            Auth::MEMBER => Err(AuthError::WrongCredentials.into()),
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
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data =
            jsonwebtoken::decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
