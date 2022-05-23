//! Queries involving users.

use super::State;
use acm::models::{Auth, User};
use log::info;

impl State {
    /// Inserta a user into the database, returning an error if unsuccessful
    pub async fn user_add(&self, user: &User) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO users (name, auth, username, password) VALUES (?, ?, ?, ?)"#,
            user.name,
            Auth::MEMBER,
            user.username,
            user.password
        )
        .execute(&self.conn)
        .await?;

        Ok(())
    }

    /// Searches for a user by username, returning their associated data if found
    pub async fn user_query(&self, username: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"SELECT name, username, password, auth as "auth: Auth", star_count FROM users WHERE username = ?"#,
            username
        )
        .fetch_one(&self.conn)
        .await
    }

    /// Searches for a user by username, returning their user id if found
    pub async fn get_user_id(&self, username: &str) -> sqlx::Result<i64> {
        let id = sqlx::query!(r#"SELECT id FROM users WHERE username = ?"#, username)
            .fetch_one(&self.conn)
            .await?
            .id;

        Ok(id)
    }
}
