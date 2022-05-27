//! This module deals with the state of the application (shocking, wow!).
//!
//! Rather than doing custom SQL queries inside the queries, we opt to define them as methods on
//! the state object given to request handlers. This decoupling should make it easier to refactor
//! the way the database handles data without altering the application logic significantly.

use crate::SqlPool;
use std::sync::Arc;

pub mod auth;
pub mod leaderboard;
pub mod meetings;
pub mod problems;
pub mod submissions;
pub mod users;

pub type AppStateRaw = std::sync::Arc<State>;
pub type AppState = actix_web::web::Data<AppStateRaw>;

pub struct State {
    pub conn: SqlPool,
    pub jwt_private_key: String,
}

impl State {
    pub async fn new(database_url: &str) -> AppStateRaw {
        let conn = SqlPool::connect(database_url).await.unwrap();

        // TODO: Implement proper JWT secret and config management
        Arc::new(State {
            conn,
            jwt_private_key: "supersecret".to_string(),
        })
    }
}
