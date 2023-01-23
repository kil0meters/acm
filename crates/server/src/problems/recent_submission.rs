use axum::{extract::Path, Extension, Json};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError, submissions::Submission};

pub async fn recent_submission(
    Path(problem_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<Option<Submission>>, ServerError> {
    claims.validate_logged_in()?;

    let submission: Option<Submission> = sqlx::query_as(
        r#"
        SELECT *
        FROM submissions
        WHERE user_id = ?
        AND problem_id = ?
        ORDER BY time DESC
        LIMIT 1
        "#,
    )
    .bind(claims.user_id)
    .bind(problem_id)
    .fetch_one(&pool)
    .await
    .ok();

    Ok(Json(submission))
}
