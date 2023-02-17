use axum::{async_trait, Extension, Json};
use reqwest::Client;
use serde_json::Value;
use shared::models::{forms::GenerateTestsJob, runner::RunnerError, test::Test};
use sqlx::SqlitePool;
use tokio::sync::broadcast::{self, Sender};

use crate::{auth::Claims, error::ServerError, ws::BroadcastMessage};

use super::{add_job, JobMap, JobQueue, JobStatus, Queueable};

pub async fn generate_tests(
    claims: Claims,
    Extension(job_queue): Extension<JobQueue>,
    Extension(job_map): Extension<JobMap>,
    Extension(broadcast): Extension<Sender<BroadcastMessage>>,
    Json(queue_item): Json<GenerateTestsJob>,
) -> Result<Json<JobStatus>, ServerError> {
    claims.validate_officer()?;

    let job = add_job(
        claims.user_id,
        job_queue,
        job_map,
        Box::new(queue_item),
        broadcast,
    )
    .await?;

    Ok(Json(job))
}

#[async_trait]
impl Queueable for GenerateTestsJob {
    async fn run(
        &self,
        ramiel_url: &str,
        _pool: &SqlitePool,
        _broadcast: &broadcast::Sender<BroadcastMessage>,
    ) -> Result<Value, ServerError> {
        let client = Client::new();
        let res = client
            .post(&format!("{ramiel_url}/generate-tests/c++"))
            .json(&self)
            .send()
            .await
            // TODO: Handle error
            .unwrap();

        let tests: Result<Vec<Test>, RunnerError> = res.json().await.unwrap();

        Ok(serde_json::to_value(tests?).unwrap())
    }

    fn info(&self) -> String {
        format!("GenerateTestsJob submitted by user {}", self.user_id)
    }

    fn job_type(&self) -> String {
        "GenerateTestsJob".to_string()
    }

    fn problem_id(&self) -> i64 {
        -1
    }
}
