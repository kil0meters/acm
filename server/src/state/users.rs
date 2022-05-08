use super::State;
use acm::models::{Auth, User};
use log::info;

impl State {
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

    pub async fn user_query(&self, username: &str) -> sqlx::Result<User> {
        info!("{:?}", username);
        sqlx::query_as!(
            User,
            r#"SELECT name, username, password, auth as "auth: Auth", star_count FROM users WHERE username = ?"#,
            username
        )
        .fetch_one(&self.conn)
        .await
    }
}
