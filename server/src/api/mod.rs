//! This module contains code for all api endpoints.

use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::json;

pub mod account;
pub mod leaderboard;
pub mod meetings;
pub mod problems;
pub mod run;
pub mod signup;
pub mod submissions;

/// A utility function for easily returning an error in a consistent format
fn api_error(code: StatusCode, error: impl Serialize) -> HttpResponse {
    HttpResponse::build(code).json(error)
}

/// A utility function for returning a JSON object whenever api_error is used
fn api_success(data: impl Serialize) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
