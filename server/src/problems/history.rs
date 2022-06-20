use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::{
    auth::Claims,
    error::{ServerError, UserError},
    submissions::Submission,
};

pub async fn history(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<Vec<Submission>>, ServerError> {
    let user_id = sqlx::query!("SELECT id FROM users WHERE username = ?", claims.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| UserError::InternalError)?
        .id;

    let submissions = sqlx::query_as!(
        Submission,
        r#"
        SELECT
            id,
            user_id,
            problem_id,
            success,
            runtime,
            error,
            time,
            code
        FROM
            submissions
        WHERE
            user_id = ? and problem_id = ?
        ORDER BY
            time DESC
        "#,
        user_id,
        problem_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(Json(submissions))
}
