use acm::models::{runner::RunnerError};
use axum::{async_trait, Extension, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::{
    auth::Claims,
    error::{ServerError},
    ws::BroadcastMessage,
};

use super::{add_job, JobMap, JobQueue, JobStatus, Queueable};

#[derive(Deserialize)]
pub struct CustomProblemInputForm {
    pub problem_id: i64,
    pub implementation: String,
    pub input: String,
}

pub async fn custom(
    Json(form): Json<CustomProblemInputForm>,
    Extension(pool): Extension<SqlitePool>,
    Extension(job_queue): Extension<JobQueue>,
    Extension(job_map): Extension<JobMap>,
    claims: Claims,
) -> Result<Json<JobStatus>, ServerError> {
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

    let queue_item = Box::new(CustomInputJob {
        problem_id: form.problem_id,
        runner,
        user_id: claims.user_id,
        implementation: form.implementation,
        reference,
        input: form.input,
    });

    let job = add_job(claims.user_id, job_queue, job_map, queue_item).await?;

    Ok(Json(job))
}

#[derive(Serialize, Debug)]
pub struct CustomInputJob {
    pub problem_id: i64,
    pub user_id: i64,
    pub runner: String,
    pub implementation: String,
    pub reference: String,
    pub input: String,
}

#[async_trait]
impl Queueable for CustomInputJob {
    async fn run(
        &self,
        ramiel_url: &str,
        _pool: &SqlitePool,
        _broadcast: &broadcast::Sender<BroadcastMessage>,
    ) -> Result<Value, ServerError> {
        let client = Client::new();
        let res = client
            .post(&format!("{ramiel_url}/custom-input/c++"))
            .json(self)
            .send()
            .await
            .map_err(|_| ServerError::InternalError)?;

        let result: Result<Value, RunnerError> = res.json().await.unwrap();

        let result = serde_json::to_value(result?).unwrap();

        Ok(result)
    }

    fn info(&self) -> String {
        format!("CustomInputJob submitted by user {}", self.user_id)
    }
}
