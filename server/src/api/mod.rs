use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::json;

pub mod auth;
pub mod leaderboard;

fn api_error(code: StatusCode, error: impl Serialize) -> HttpResponse {
    HttpResponse::build(code).json(json!({ "error": error }))
}

fn api_success(data: impl Serialize) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
