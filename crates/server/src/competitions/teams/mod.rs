use axum::{
    routing::{get, post},
    Router,
};
use serde::Serialize;

use crate::auth::User;

mod index;
mod join;
mod joinable;
mod leave;
mod me;
mod new;
mod team;

#[derive(Serialize)]
pub struct Team {
    id: i64,
    name: String,
    members: Vec<User>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/new", post(new::new))
        .route("/join", post(join::join))
        .route("/leave", post(leave::leave))
        .route("/joinable", get(joinable::joinable))
        .route("/", get(index::teams))
        .route("/me", get(me::me))
        .route("/:team_id", get(team::team))
}
