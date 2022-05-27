//! API endpoints relating to the leaderboard

use actix_web::{get, web::Json, Responder};

use crate::state::AppState;

#[get("/leaderboard")]
pub async fn leaderboard() -> impl Responder {
    return "";
}

/* #[get("/leaderboard/total-submissions")]
async fn total_submissions() -> impl Responder {} */

#[get("/leaderboard/first-place")]
async fn first_place_finishes(state: AppState) -> impl Responder {
    Json(state.first_place_submissions().await)
}
