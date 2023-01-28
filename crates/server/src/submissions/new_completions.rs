use axum::{extract::Query, Extension, Json};
use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use sqlx::SqlitePool;

use super::Submission;

#[derive(Deserialize)]
pub struct NewCompletionsForm {
    since: Option<NaiveDateTime>,
}

pub async fn new_completions(
    Extension(pool): Extension<SqlitePool>,
    Query(query): Query<NewCompletionsForm>,
) -> Json<Vec<Submission>> {
    let since = query.since.unwrap_or_else(|| Utc::now().naive_local());

    let submissions: Vec<Submission> = sqlx::query_as(
        r#"
        SELECT
            id,
            problem_id,
            user_id,
            success,
            runtime,
            error,
            code,
            min(time) as time
        FROM
            submissions
        WHERE
            success = true AND DATETIME(time) > DATETIME(?, 'localtime')
        GROUP BY
            user_id,
            problem_id
        "#,
    )
    .bind(since)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(submissions)
}
