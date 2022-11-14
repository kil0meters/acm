use axum::{extract::Query, Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError, pagination::Pagination};

use super::Problem;

#[derive(Deserialize)]
pub struct ProblemOptions {
    competition_id: Option<i64>,
}

pub async fn problems(
    Extension(pool): Extension<SqlitePool>,
    Query(options): Query<ProblemOptions>,
    Query(pagination): Query<Pagination<0, 10>>,
    claims: Claims,
) -> Result<Json<Vec<Problem>>, ServerError> {
    let is_officer = claims.validate_officer().is_ok();

    let problems = if options.competition_id.is_some() {
        if is_officer {
            sqlx::query_as!(
                Problem,
                r#"
                SELECT
                    id,
                    title,
                    description,
                    runner,
                    template,
                    competition_id
                FROM problems
                WHERE competition_id = ?
                ORDER BY update_dt DESC
                LIMIT ? OFFSET ?
                "#,
                options.competition_id,
                pagination.count,
                pagination.offset
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
                    template,
                    competition_id
                FROM problems
                WHERE visible = true
                AND competition_id = ?
                ORDER BY update_dt DESC
                LIMIT ? OFFSET ?
                "#,
                options.competition_id,
                pagination.count,
                pagination.offset
            )
            .fetch_all(&pool)
            .await
            .map_err(|_| ServerError::InternalError)?
        }
    } else {
        if is_officer {
            sqlx::query_as!(
                Problem,
                r#"
                SELECT
                    id,
                    title,
                    description,
                    runner,
                    template,
                    competition_id
                FROM problems
                WHERE competition_id is NULL
                ORDER BY update_dt DESC
                LIMIT ? OFFSET ?
                "#,
                pagination.count,
                pagination.offset
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
                    template,
                    competition_id
                FROM problems
                WHERE visible = true
                AND competition_id is NULL
                ORDER BY update_dt DESC
                LIMIT ? OFFSET ?
                "#,
                pagination.count,
                pagination.offset
            )
            .fetch_all(&pool)
            .await
            .map_err(|_| ServerError::InternalError)?
        }
    };

    Ok(Json(problems))
}
