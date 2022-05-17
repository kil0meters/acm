//! Handles authentication

use acm::models::Auth;
use actix_web::{dev::ServiceRequest, HttpMessage, Result};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
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

/// Validates incoming requests, only allowing users with a valid JWT to proceed.
///
/// TODO: In the future, rather than having the "/authorized/" prefix to such endpoints, we should
/// wrap everything in this middleware, and instead supply an Option<Claims> to ReqData.
pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest> {
    let state = req.app_data::<AppState>().unwrap();

    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match state.validate_token(credentials.token()) {
        Some(claims) => {
            {
                // Passes through the Claims the request can attest to so they can be used in the
                // request by ReqData<Claims>.
                let mut extensions = req.extensions_mut();
                extensions.insert(claims);
            }

            Ok(req)
        }
        None => Err(AuthenticationError::from(config).into()),
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