use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::json;

pub mod leaderboard;
pub mod problems;
pub mod signup;

fn api_error(code: StatusCode, error: impl Serialize) -> HttpResponse {
    HttpResponse::build(code).json(json!({ "error": error }))
}

fn api_success(data: impl Serialize) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
