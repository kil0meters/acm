use axum::{async_trait, Extension, Json};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use shared::models::{forms::CustomInputJob, runner::RunnerError};
use sqlx::SqlitePool;
use tokio::sync::broadcast::{self, Sender};
use wasm_memory::WasmFunctionCall;

use crate::{auth::Claims, error::ServerError, ws::BroadcastMessage};

use super::{add_job, JobMap, JobQueue, JobStatus, Queueable};

#[derive(Deserialize)]
pub struct CustomProblemInputForm {
    pub problem_id: i64,
    pub implementation: String,
    pub input: WasmFunctionCall,
}

pub async fn custom(
    claims: Claims,
    Extension(pool): Extension<SqlitePool>,
    Extension(job_queue): Extension<JobQueue>,
    Extension(job_map): Extension<JobMap>,
    Extension(broadcast): Extension<Sender<BroadcastMessage>>,
    Json(form): Json<CustomProblemInputForm>,
) -> Result<Json<JobStatus>, ServerError> {
    claims.validate_logged_in()?;

    let (reference, runtime_multiplier): (String, Option<f64>) = sqlx::query_as(
        r#"
        SELECT
            reference,
            runtime_multiplier
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
        user_id: claims.user_id,
        implementation: form.implementation,
        runtime_multiplier,
        reference,
        input: form.input,
    });

    let job = add_job(claims.user_id, job_queue, job_map, queue_item, broadcast).await?;

    Ok(Json(job))
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

    fn job_type(&self) -> String {
        "CustomInputJob".to_string()
    }

    fn problem_id(&self) -> i64 {
        self.problem_id
    }
}
