use axum::{extract::Path, Extension};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

pub async fn invalidate(
    Path(submission_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<(), ServerError> {
    claims.validate_officer()?;

    log::info!(
        "User {} manually invalidating submission {}",
        claims.user_id,
        submission_id
    );

    sqlx::query!(
        r#"
        UPDATE submissions
        SET success = false,
            error = 'Manually overridden by officer'
        WHERE id = ?"#,
        submission_id
    )
    .execute(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    Ok(())
}
