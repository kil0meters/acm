use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Problem;

pub async fn problem(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Problem>, ServerError> {
    let problem = sqlx::query_as!(
        Problem,
        r#"
        SELECT
            id,
            title,
            description,
            runner,
            template
        FROM
            problems
        WHERE
            id = ?
        "#,
        problem_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(problem))
}
