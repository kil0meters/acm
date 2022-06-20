use acm::models::{
    forms::RunnerForm,
    runner::{RunnerError, RunnerResponse},
    test::Test,
};
use axum::{Extension, Json};
use reqwest::Client;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{auth::Claims, error::ServerError, submissions::Submission};

#[derive(Deserialize)]
pub struct SubmitForm {
    pub problem_id: i64,
    pub implementation: String,
}

pub async fn submit(
    Json(form): Json<SubmitForm>,
    Extension(pool): Extension<SqlitePool>,
    Extension(ramiel_url): Extension<String>,
    claims: Claims,
) -> Result<Json<Submission>, ServerError> {
    let runner = sqlx::query!(
        r#"SELECT runner FROM problems WHERE id = ?"#,
        form.problem_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?
    .runner;

    let tests = sqlx::query_as!(
        Test,
        r#"
        SELECT
            id,
            test_number as [index],
            input,
            expected_output
        FROM
            tests
        WHERE
            problem_id = ?"#,
        form.problem_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    let client = Client::new();
    let res = client
        .post(&format!("{ramiel_url}/run/c++"))
        .json(&RunnerForm {
            problem_id: form.problem_id,
            username: claims.username.clone(),
            runner,
            implementation: form.implementation.clone(),
            tests,
        })
        .send()
        .await
        .map_err(|_| ServerError::InternalError)?;

    let res: Result<RunnerResponse, RunnerError> =
        res.json().await.map_err(|_| ServerError::InternalError)?;

    let user_id = sqlx::query!("SELECT id FROM users WHERE username = ?", claims.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| ServerError::NotFound)?
        .id;

    let (passed, runtime, error, tests) = match res {
        Ok(res) => (res.passed, res.runtime, None, res.tests),
        Err(err) => (false, 0, Some(err.to_string()), vec![]),
    };

    let mut tx = pool.begin().await.unwrap();
    let submission = sqlx::query_as!(
        Submission,
        r#"
        INSERT INTO submissions (
            problem_id,
            user_id,
            success,
            runtime,
            error,
            code
        )
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
        form.problem_id,
        user_id,
        passed,
        runtime,
        error,
        form.implementation
    )
    .fetch_one(&mut tx)
    .await
    .map_err(|_| ServerError::InternalError)?;

    for test in &tests {
        sqlx::query!(
            r#"
            INSERT INTO test_results (
                submission_id,
                test_id,
                runtime,
                output,
                success
            ) VALUES (?, ?, ?, ?, ?)
            "#,
            submission.id,
            test.id,
            test.runtime,
            test.output,
            passed,
        )
        .execute(&mut tx)
        .await
        .map_err(|_| ServerError::InternalError)?;
    }

    tx.commit().await.map_err(|_| ServerError::InternalError)?;

    Ok(Json(submission))
}
