use acm::models::test::TestResult;
use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::{error::ServerError, MAX_TEST_LENGTH};

pub async fn tests(
    Path(submission_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<TestResult>>, ServerError> {
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
            test_results.submission_id = ?
        ORDER BY
            tests.test_number ASC, test_results.success
        "#,
        submission_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    tests.iter_mut().for_each(|test| {
        test.truncate(MAX_TEST_LENGTH);
    });

    Ok(Json(tests))
}
