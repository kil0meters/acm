use super::State;
use acm::models::{forms::CreateProblemForm, Problem};
use sqlx::Row;

impl State {
    pub async fn problem_add(&self, problem: &CreateProblemForm) -> sqlx::Result<i64> {
        sqlx::query!(
            r#"INSERT INTO problems (title, description, runner, template, visible) VALUES (?, ?, ?, ?, ?)"#,
            problem.title,
            problem.description,
            problem.runner,
            problem.template,
            true
        )
        .execute(&self.conn)
        .await?;

        let id = sqlx::query!("SELECT id FROM problems WHERE title = ?", problem.title)
            .fetch_one(&self.conn)
            .await?
            .id;

        Ok(id)
    }

    // TODO: Should only fetch visible problems based on the user's authentication
    // As times goes on, filtering after querying will become increasingly inefficient.
    pub async fn problems_get(&self) -> Vec<Problem> {
        sqlx::query_as!(
            Problem,
            r#"SELECT id, title, description, runner, template, visible FROM problems"#
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_else(|_| Vec::new())
    }

    pub async fn problems_get_by_id(&self, id: u32) -> Option<Problem> {
        sqlx::query_as!(
            Problem,
            r#"SELECT id, title, description, runner, template, visible FROM problems WHERE id = ?"#,
            id
        )
        .fetch_one(&self.conn)
        .await
        .ok()
    }
}
