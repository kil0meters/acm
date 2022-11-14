use acm::models::test::TestResult;
use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError, MAX_TEST_LENGTH};

pub async fn recent_tests(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<Vec<TestResult>>, ServerError> {
    claims.validate_logged_in()?;

    let mut tests = sqlx::query_as!(
        TestResult,
        r#"
        SELECT
            test_results.id as id,
            test_results.success as success,
            test_results.output as output,
            test_results.runtime as runtime,
            test_results.error as error,
            tests.max_runtime as max_runtime,
            tests.input as input,
            tests.expected_output as expected_output,
            tests.test_number as [index]
        FROM
            test_results INNER JOIN tests
            ON test_results.test_id = tests.id
        WHERE
            test_results.submission_id = (
                SELECT id
                FROM submissions
                WHERE user_id = ?
                AND problem_id = ?
                ORDER BY time DESC
                LIMIT 1
            )
        ORDER BY
            tests.test_number ASC, test_results.success
        "#,
        claims.user_id,
        problem_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    tests.iter_mut().for_each(|test| {
        test.truncate(MAX_TEST_LENGTH);
    });

    Ok(Json(tests))
}
