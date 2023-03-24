use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use shared::models::test::Test;
use sqlx::{FromRow, SqlitePool};

use crate::error::ServerError;

#[derive(FromRow, Serialize)]
pub struct TestNoInput {
    id: i64,
    #[sqlx(rename = "test_number")]
    index: i64,
    hidden: bool,
}

pub async fn tests(
    Extension(pool): Extension<SqlitePool>,
    Path(problem_id): Path<i64>,
) -> Result<Json<Vec<TestNoInput>>, ServerError> {
    let tests: Vec<TestNoInput> = sqlx::query_as(
        r#"
        SELECT
            id,
            test_number,
            hidden
        FROM
            tests
        WHERE
            problem_id = ?"#,
    )
    .bind(problem_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        log::error!("{e}");
        ServerError::NotFound
    })?;

    Ok(Json(tests))
}

pub async fn problem_test(
    Extension(pool): Extension<SqlitePool>,
    Path((problem_id, test_number)): Path<(i64, i64)>,
) -> Result<Json<Option<Test>>, ServerError> {
    // check if the test is hidden
    let (hidden,): (bool,) =
        sqlx::query_as("SELECT hidden FROM tests WHERE test_number = ? AND problem_id = ?")
            .bind(test_number)
            .bind(problem_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                log::error!("{e}");
                ServerError::NotFound
            })?;

    log::info!("hidden: {hidden}");

    if hidden {
        return Ok(Json(None));
    }

    let test: Test = sqlx::query_as(
        r#"
        SELECT
            id,
            test_number,
            input,
            expected_output,
            max_runtime,
            test_number
        FROM
            tests
        WHERE
            problem_id = ?
        AND
            test_number = ?"#,
    )
    .bind(problem_id)
    .bind(test_number)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        log::error!("{e}");
        ServerError::NotFound
    })?;

    Ok(Json(Some(test)))
}
