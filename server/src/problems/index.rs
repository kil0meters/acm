use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::error::ServerError;

use super::Problem;

pub async fn problems(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Problem>>, ServerError> {
    let problems = sqlx::query_as!(
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
        ORDER BY
            update_dt DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::InternalError)?;

    Ok(Json(problems))
}
