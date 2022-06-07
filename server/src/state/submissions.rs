use acm::models::{
    runner::{RunnerError, RunnerResponse},
    test::TestResult,
    Submission,
};
use sqlx::{Sqlite, Transaction};

use super::State;

impl State {
    pub async fn recent_submissions_for_user(
        &self,
        username: &str,
        count: u32,
        offset: u32,
    ) -> sqlx::Result<Vec<Submission>> {
        let user_id = self.get_user_id(username).await?;

        sqlx::query_as!(
            Submission,
            r#"
            SELECT
                id,
                problem_id,
                user_id,
                success,
                runtime,
                time,
                error,
                code
            FROM
                submissions
            WHERE
                user_id = ?
            ORDER BY
                time DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            count,
            offset
        )
        .fetch_all(&self.conn)
        .await
    }

    pub async fn save_submission(
        &self,
        res: &Result<RunnerResponse, RunnerError>,
        code: &str,
        username: &str,
        problem_id: i64,
    ) -> sqlx::Result<Submission> {
        let user_id = self.get_user_id(&username).await?;
        let mut tx = self.conn.begin().await?;

        let submission = match res {
            Ok(res) => {
                let submission = sqlx::query_as!(
                    Submission,
                    r#"
                    INSERT INTO submissions (
                        problem_id,
                        user_id,
                        success,
                        runtime,
                        code
                    )
                    VALUES (?, ?, ?, ?, ?)
                    RETURNING *
                    "#,
                    problem_id,
                    user_id,
                    res.passed,
                    res.runtime,
                    code
                )
                .fetch_one(&mut tx)
                .await?;

                log::info!("submission id: {}", submission.id);

                for test in &res.tests {
                    Self::insert_test_result(&mut tx, test, submission.id, test.success).await?;
                }

                submission
            }
            Err(e) => {
                let e = e.to_string();

                let submission = sqlx::query_as!(
                    Submission,
                    r#"
                    INSERT INTO submissions (
                        problem_id,
                        user_id,
                        success,
                        runtime,
                        error,
                        code
                    ) VALUES (?, ?, ?, ?, ?, ?)
                    RETURNING *"#,
                    problem_id,
                    user_id,
                    false,
                    0i64,
                    e,
                    code
                )
                .fetch_one(&mut tx)
                .await?;

                submission
            }
        };

        tx.commit().await?;

        Ok(submission)
    }

    pub async fn tests_for_submission(&self, submission_id: i64) -> Vec<TestResult> {
        sqlx::query_as!(
            TestResult,
            r#"
            SELECT
                test_results.id as id,
                test_results.success as success,
                test_results.output as output,
                test_results.runtime as runtime,
                tests.input as input,
                tests.expected_output as expected_output,
                tests.test_number as [index]
            FROM
                test_results INNER JOIN tests
                ON test_results.test_id = tests.id
            WHERE
                test_results.submission_id = ?
            ORDER BY
                tests.test_number ASC, test_results.success
            "#,
            submission_id
        )
        .fetch_all(&self.conn)
        .await
        .unwrap_or_default()
    }

    pub async fn get_submission(&self, submission_id: i64) -> sqlx::Result<Submission> {
        sqlx::query_as!(
            Submission,
            r#"
            SELECT
                *
            FROM
                submissions
            WHERE
                id = ?
            "#,
            submission_id
        )
        .fetch_one(&self.conn)
        .await
    }

    async fn insert_test_result(
        tx: &mut Transaction<'_, Sqlite>,
        test: &TestResult,
        submission_id: i64,
        passed: bool,
    ) -> sqlx::Result<()> {
        log::info!("{}", test.id);

        sqlx::query!(
            r#"
            INSERT INTO test_results (
                submission_id,
                test_id,
                runtime,
                output,
                success
            ) VALUES (?, ?, ?, ?, ?)
            "#,
            submission_id,
            test.id,
            test.runtime,
            test.output,
            passed,
        )
        .execute(tx)
        .await
        .unwrap();

        Ok(())
    }
}
