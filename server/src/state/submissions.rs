use acm::models::{
    Submission,
    runner::{RunnerError, RunnerResponse},
    test::TestResult,
};

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
                problem_id,
                success,
                runtime,
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
        ).fetch_all(&self.conn).await
    }

    pub async fn save_submission(
        &self,
        res: &Result<RunnerResponse, RunnerError>,
        code: &str,
        username: &str,
        problem_id: i64,
    ) -> sqlx::Result<()> {
        let user_id = self.get_user_id(&username).await?;

        match res {
            Ok(res) => {
                let passed = res.failed_tests.is_empty();

                let submission_id = sqlx::query!(
                    r#"
                    INSERT INTO submissions (
                        problem_id,
                        user_id,
                        success,
                        runtime,
                        code
                    )
                    VALUES (?, ?, ?, ?, ?)
                    RETURNING id
                    "#,
                    problem_id,
                    user_id,
                    passed,
                    res.runtime,
                    code
                )
                .fetch_one(&self.conn)
                .await?
                .id;

                log::info!("submission id: {}", submission_id);

                for test in &res.failed_tests {
                    self.insert_test_result(test, submission_id, false).await?;
                }

                for test in &res.passed_tests {
                    self.insert_test_result(test, submission_id, true).await?;
                }
            }
            Err(e) => {
                let e = e.to_string();

                sqlx::query!(
                    r#"
                    INSERT INTO submissions (
                        problem_id,
                        user_id,
                        success,
                        runtime,
                        error,
                        code
                    ) VALUES (?, ?, ?, ?, ?, ?)"#,
                    problem_id,
                    user_id,
                    false,
                    0i64,
                    e,
                    code
                )
                .execute(&self.conn)
                .await?;
            }
        };

        Ok(())
    }

    pub async fn insert_test_result(
        &self,
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
                expected_output,
                success
            ) VALUES (?, ?, ?, ?, ?)
            "#,
            submission_id,
            test.id,
            test.time,
            test.expected_output,
            passed,
        )
        .execute(&self.conn)
        .await
        .unwrap();

        Ok(())
    }
}
