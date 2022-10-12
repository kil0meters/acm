use acm::models::test::Test;
use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::{error::ServerError, MAX_TEST_LENGTH};

pub async fn tests(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Test>>, ServerError> {
    let mut tests = sqlx::query_as!(
        Test,
        r#"
        SELECT
            id,
            test_number as [index],
            max_runtime,
            input,
            expected_output
        FROM
            tests
        WHERE
            problem_id = ?"#,
        problem_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    tests
        .iter_mut()
        .for_each(|test| test.truncate(MAX_TEST_LENGTH));

    Ok(Json(tests))
}
