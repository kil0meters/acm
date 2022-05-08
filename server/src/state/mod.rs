use crate::SqlPool;
use std::sync::Arc;

pub mod users;

pub type AppStateRaw = std::sync::Arc<State>;
pub type AppState = actix_web::web::Data<AppStateRaw>;

pub struct State {
    pub conn: SqlPool,
    pub jwt_private_key: String,
}

impl State {
    pub async fn new_state() -> AppStateRaw {
        let conn = SqlPool::connect("./db.sqlite").await.unwrap();

        // TODO: Implement proper JWT secret and config management
        Arc::new(State {
            conn,
            jwt_private_key: "supersecret".to_string(),
        })
    }

    // pub async fn user_add() {}
}
