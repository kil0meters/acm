use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

use super::{Difficulty, Problem};

pub async fn problem(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<Problem>, ServerError> {
    let problem = sqlx::query_as!(
        Problem,
        r#"
        SELECT
            id,
            title,
            description,
            runner,
            template,
            competition_id,
            visible,
            runtime_multiplier,
            difficulty as "difficulty: Difficulty"
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

    if problem.visible || claims.validate_officer().is_ok() {
        Ok(Json(problem))
    } else {
        Err(ServerError::NotFound)
    }
}
