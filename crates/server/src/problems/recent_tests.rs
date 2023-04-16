use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use shared::models::test::TestResult;
use sqlx::{FromRow, SqlitePool};

use crate::{auth::Claims, error::ServerError};

#[derive(FromRow, Serialize)]
pub struct TestResultNoInput {
    id: i64,
    index: i64,
    success: bool,
    hidden: bool,
}

pub async fn recent_tests(
    claims: Claims,
    Extension(pool): Extension<SqlitePool>,
    Path(problem_id): Path<i64>,
) -> Result<Json<Vec<TestResultNoInput>>, ServerError> {
    claims.validate_logged_in()?;

    let tests: Vec<TestResultNoInput> = sqlx::query_as(
        r#"
        SELECT
            test_results.id as id,
            tests.test_number as [index],
            tests.hidden as hidden,
            test_results.success as success
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
    )
    .bind(claims.user_id)
    .bind(problem_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        log::error!("{e:?}");

        ServerError::NotFound
    })?;

    Ok(Json(tests))
}

pub async fn recent_tests_test(
    claims: Claims,
    Extension(pool): Extension<SqlitePool>,
    Path((problem_id, test_number)): Path<(i64, i64)>,
) -> Result<Json<TestResult>, ServerError> {
    claims.validate_logged_in()?;

    let (runtime_multiplier,): (Option<f64>,) =
        sqlx::query_as(r#"SELECT runtime_multiplier FROM problems WHERE id = ?"#)
            .bind(problem_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| ServerError::NotFound)?;

    let mut test: TestResult = sqlx::query_as(
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
            tests.test_number as test_number,
            tests.hidden as hidden
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
        AND
            test_number = ?
        ORDER BY
            tests.test_number ASC, test_results.success"#,
    )
    .bind(claims.user_id)
    .bind(problem_id)
    .bind(test_number)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        log::error!("{e}");
        ServerError::NotFound
    })?;

    test.adjust_runtime(runtime_multiplier);

    Ok(Json(test))
}
