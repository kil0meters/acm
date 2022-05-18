//! Queries involving problems

use super::State;
use acm::models::{forms::CreateProblemForm, Problem};

impl State {
    /// Adds a problem to the database, returning the id of the problem or an error.
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

        for test in &problem.tests {
            sqlx::query!(
                r#"INSERT INTO tests (problem_id, test_number, input, expected_output) VALUES (?, ?, ?, ?)"#,
                id,
                test.index,
                test.input,
                test.expected_output
            )
            .execute(&self.conn)
            .await?;
        }

        Ok(id)
    }

    /// Fetches all problems from the database and returns them.
    pub async fn problems_get(&self) -> Vec<Problem> {
        // TODO: Should only fetch visible problems based on the user's authentication
        // As times goes on, filtering after querying will become increasingly inefficient.

        sqlx::query_as!(
            Problem,
            r#"SELECT id, title, description, runner, template, visible FROM problems"#
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_else(|_| Vec::new())
    }

    /// Searches the database for a problem with a given id, returning None if not found.
    pub async fn problems_get_by_id(&self, id: i64) -> Option<Problem> {
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
