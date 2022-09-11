use acm::models::{forms::RunnerCustomProblemInputForm, runner::RunnerError, test::TestResult};
use axum::{Extension, Json};
use reqwest::Client;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError};

#[derive(Deserialize)]
pub struct CustomProblemInputForm {
    pub problem_id: i64,
    pub implementation: String,
    pub input: String,
}

pub async fn custom(
    Json(form): Json<CustomProblemInputForm>,
    Extension(ramiel_url): Extension<String>,
    Extension(pool): Extension<SqlitePool>,
    claims: Claims,
) -> Result<Json<TestResult>, ServerError> {
    let client = Client::new();

    let (runner, reference): (String, String) = sqlx::query_as(
        r#"
        SELECT
            runner,
            reference
        FROM
            problems
        WHERE
            id = ?
        "#,
    )
    .bind(form.problem_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    let res = client
        .post(&format!("{ramiel_url}/custom-input/c++"))
        .json(&RunnerCustomProblemInputForm {
            problem_id: form.problem_id,
            runner,
            user_id: claims.user_id,
            implementation: form.implementation,
            reference,
            input: form.input,
        })
        .send()
        .await
        // TODO: Handle error
        .unwrap();

    let result: Result<TestResult, RunnerError> = res.json().await.unwrap();

    Ok(Json(result?))
}
