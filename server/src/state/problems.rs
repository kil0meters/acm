//! Queries involving problems

use super::State;
use acm::models::{forms::CreateProblemForm, test::Test, Problem, Submission};

impl State {
    /// Adds a problem to the database, returning the id of the problem or an error.
    pub async fn problem_add(&self, problem: &CreateProblemForm) -> sqlx::Result<i64> {
        let mut tx = self.conn.begin().await?;

        let id = sqlx::query!(
            r#"
            INSERT INTO problems (
                title,
                description,
                runner,
                reference,
                template,
                activity_id
            ) VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            problem.title,
            problem.description,
            problem.runner,
            problem.reference,
            problem.template,
            problem.activity_id
        )
        .fetch_one(&mut tx)
        .await?
        .id;

        for test in &problem.tests {
            sqlx::query!(
                r#"
                INSERT INTO tests (
                    problem_id,
                    test_number,
                    input,
                    expected_output
                )
                VALUES (?, ?, ?, ?)
                "#,
                id,
                test.index,
                test.input,
                test.expected_output
            )
            .execute(&mut tx)
            .await?;
        }

        tx.commit().await?;

        Ok(id)
    }

    /// Fetches all problems from the database and returns them.
    pub async fn problems_get(&self) -> Vec<Problem> {
        sqlx::query_as!(
            Problem,
            r#"
            SELECT
                id,
                title,
                description,
                runner,
                reference,
                template,
                visible
            FROM
                problems
            ORDER BY
                update_dt DESC"#
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_else(|_| Vec::new())
    }

    /// Searches the database for a problem with a given id, returning None if not found.
    pub async fn problems_get_by_id(&self, id: i64) -> Option<Problem> {
        sqlx::query_as!(
            Problem,
            r#"
            SELECT
                id,
                title,
                description,
                runner,
                reference,
                template,
                visible
            FROM
                problems
            WHERE
                id = ?
            "#,
            id
        )
        .fetch_one(&self.conn)
        .await
        .ok()
    }

    /// Receives all tests for a given problem
    pub async fn tests_get_for_problem_id(&self, problem_id: i64) -> Vec<Test> {
        sqlx::query_as!(
            Test,
            r#"
            SELECT
                id,
                test_number as [index],
                input,
                expected_output
            FROM
                tests
            WHERE
                problem_id = ?"#,
            problem_id
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_else(|_| vec![])
    }

    /// Gets problem history for a particular user
    pub async fn problem_history(&self, problem_id: i64, user_id: i64) -> Vec<Submission> {
        sqlx::query_as!(
            Submission,
            r#"
            SELECT
                id,
                user_id,
                problem_id,
                success,
                runtime,
                error,
                time,
                code
            FROM
                submissions
            WHERE
                user_id = ? and problem_id = ?
            ORDER BY
                time DESC
            "#,
            user_id,
            problem_id
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_else(|_| vec![])
    }
}
