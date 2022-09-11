use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    error::{ServerError, UserError},
    submissions::Submission
};

#[derive(Deserialize)]
pub struct Pagination<const C: i64, const O: i64> {
    #[serde(default = "default_value::<O>")]
    count: i64,

    #[serde(default = "default_value::<C>")]
    offset: i64,
}

fn default_value<const T: i64>() -> i64 {
    T
}

pub async fn submissions(
    Path(username): Path<String>,
    Query(pagination): Query<Pagination<0, 10>>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Submission>>, ServerError> {
    let user_id = sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE username = ?
        "#,
        username
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| UserError::NotFound(username))?
    .id;

    let submissions = sqlx::query_as!(
        Submission,
        r#"
        SELECT
            id,
            problem_id,
            user_id,
            success,
            runtime,
            error,
            time,
            code
        FROM
            submissions
        WHERE
            user_id = ?
        ORDER BY
            time DESC
        LIMIT ? OFFSET ?
        "#,
        user_id,
        pagination.count,
        pagination.offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| UserError::InternalError)?;

    Ok(Json(submissions))
}
