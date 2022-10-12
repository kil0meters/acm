use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

use super::Problem;

pub async fn problems(
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<Vec<Problem>>, ServerError> {
    let is_officer = claims.validate_officer().is_ok();

    let problems = if is_officer {
        sqlx::query_as!(
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
        .map_err(|_| ServerError::InternalError)?
    } else {
        sqlx::query_as!(
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
                visible = true
            ORDER BY
                update_dt DESC
            "#
        )
        .fetch_all(&pool)
        .await
        .map_err(|_| ServerError::InternalError)?
    };

    Ok(Json(problems))
}
