//! Handles authentication

use acm::models::Auth;
use actix_web::{
    dev::Payload,
    Error, Result, FromRequest, HttpRequest, error::ErrorNotFound,
};
use std::future::{Ready, self};
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

use super::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub username: String,
    pub exp: usize,
    pub auth: Auth,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = req.headers();

        if let Some(auth_string) = headers.get("Authorization") {
            let token = auth_string.to_str().unwrap().to_string();
            let token = token.trim_start_matches("Bearer ");

            let state = req.app_data::<AppState>().unwrap();

            match state.validate_token(token) {
                Some(claims) => {
                    future::ready(Ok(claims))
                }
                None => {
                    future::ready(Err(ErrorNotFound("Invalid credentials")))
                }
            }
        } else {
            future::ready(Err(ErrorNotFound("wowee")))
        }
    }
}

impl State {
    /// Attempts to validate a set of Claims, returning them if they were retreived. Otherwise
    /// returns None.
    fn validate_token(&self, token: &str) -> Option<Claims> {
        let key = self.jwt_private_key.as_bytes();
        jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(key), &Default::default())
            .ok()
            .map(|e| e.claims)
    }

    /// Creates a JWT from a set of Claims.
    pub fn create_token(&self, claims: &Claims) -> String {
        let key = self.jwt_private_key.as_bytes();
        jsonwebtoken::encode(&Header::default(), claims, &EncodingKey::from_secret(key)).unwrap()
    }
}
