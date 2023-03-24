// use axum::{extract::Path, Extension, Json};
// use shared::models::test::TestResult;
// use sqlx::SqlitePool;
//
// use crate::error::ServerError;
//
// pub async fn tests(
//     Extension(pool): Extension<SqlitePool>,
//     Path(submission_id): Path<i64>,
// ) -> Result<Json<Vec<TestResult>>, ServerError> {
//     let tests: Vec<TestResult> = sqlx::query_as(
//         r#"
//         SELECT
//             test_results.id as id,
//             test_results.success as success,
//             test_results.output as output,
//             test_results.runtime as runtime,
//             test_results.error as error,
//             tests.max_runtime as max_runtime,
//             tests.input as input,
//             tests.expected_output as expected_output,
//             tests.test_number as [index]
//         FROM
//             test_results INNER JOIN tests
//             ON test_results.test_id = tests.id
//         WHERE
//             test_results.submission_id = ?
//         ORDER BY
//             tests.test_number ASC, test_results.success
//         "#,
//     )
//     .bind(submission_id)
//     .fetch_all(&pool)
//     .await
//     .map_err(|_| ServerError::NotFound)?;
//
//     Ok(Json(tests))
// }
