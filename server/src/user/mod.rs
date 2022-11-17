use axum::{
    routing::{get, post},
    Router,
};

mod edit;
mod me;
mod star_count;
mod submissions;
mod user_info;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/username/:username/submissions",
            get(submissions::submissions),
        )
        .route("/username/:username", get(user_info::username))
        .route("/id/:id", get(user_info::id))
        .route("/star-count/:id", get(star_count::star_count))
        .route("/edit/:user_id", post(edit::edit))
        .route("/me", get(me::me))
}
