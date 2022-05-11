use acm::models::Auth;
use actix_web::{dev::ServiceRequest, HttpMessage, Result};
use actix_web_httpauth::{
    extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use jsonwebtoken::{EncodingKey, DecodingKey, Header};
use log::info;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

use super::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub username: String,
    pub exp: usize,
    pub auth: Auth,
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest> {
    let state = req.app_data::<AppState>().unwrap();

    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match state.validate_token(credentials.token()) {
        Some(claims) => {
            {
                let mut extensions = req.extensions_mut();
                extensions.insert(claims);
            }

            info!("what");

            Ok(req)
        }
        None => Err(AuthenticationError::from(config).into()),
    }
}

impl State {
    fn validate_token(&self, token: &str) -> Option<Claims> {
        let key = self.jwt_private_key.as_bytes();
        jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(key),
            &Default::default()
        ).ok().map(|e| e.claims)
    }

    pub fn create_token(&self, claims: &Claims) -> String {
        let key = self.jwt_private_key.as_bytes();
        jsonwebtoken::encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(key)
        ).unwrap()
    }
}
