use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

mod first_place;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromRow)]
pub struct LeaderboardItem {
    pub username: String,
    pub name: String,
    pub count: i64,
}

pub fn routes() -> Router {
    Router::new().route("/first-place", get(first_place::first_place))
}
