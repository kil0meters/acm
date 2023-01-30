use axum::{async_trait, Extension, Json};
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use shared::models::{
    forms::SubmitJob,
    runner::{RunnerError, RunnerResponse},
    test::Test,
};
use sqlx::SqlitePool;
use tokio::sync::broadcast::Sender;

use crate::{auth::Claims, error::ServerError, submissions::Submission, ws::BroadcastMessage};

use super::{add_job, JobMap, JobQueue, JobStatus, Queueable};

#[derive(Deserialize)]
pub struct SubmitForm {
    pub problem_id: i64,
    pub implementation: String,
}

pub async fn submit(
    Extension(pool): Extension<SqlitePool>,
    Extension(job_queue): Extension<JobQueue>,
    Extension(job_map): Extension<JobMap>,
    Extension(broadcast): Extension<Sender<BroadcastMessage>>,
    claims: Claims,
    Json(form): Json<SubmitForm>,
) -> Result<Json<JobStatus>, ServerError> {
    log::info!("{:?}", claims);

    claims.validate_logged_in()?;

    let tests: Vec<Test> = sqlx::query_as(
        r#"
        SELECT
            id,
            test_number,
            input,
            max_runtime,
            expected_output
        FROM
            tests
        WHERE
            problem_id = ?"#,
    )
    .bind(form.problem_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| ServerError::NotFound)?;

    let queue_item = Box::new(SubmitJob {
        problem_id: form.problem_id,
        user_id: claims.user_id,
        implementation: form.implementation.clone(),
        tests,
    });

    let job = add_job(claims.user_id, job_queue, job_map, queue_item, broadcast).await?;

    Ok(Json(job))
}

#[async_trait]
impl Queueable for SubmitJob {
    async fn run(
        &self,
        ramiel_url: &str,
        pool: &SqlitePool,
        broadcast: &Sender<BroadcastMessage>,
    ) -> Result<Value, ServerError> {
        let client = Client::new();
        let res = client
            .post(&format!("{ramiel_url}/run/c++"))
            .json(self)
            .send()
            .await
            .map_err(|_| ServerError::InternalError)?;

        let res: Result<RunnerResponse, RunnerError> =
            res.json().await.map_err(|_| ServerError::InternalError)?;

        let (passed, runtime, error, tests) = match res {
            Ok(res) => (res.passed, res.runtime, None, res.tests),
            Err(err) => (false, 0, Some(err.to_string()), vec![]),
        };

        // find the asymptotic complexity
        let complexity = if passed {
            let inputs = tests.iter().map(|test| test.input.clone()).collect();
            let times = tests.iter().map(|test| test.fuel as f32).collect();
            wasm_memory::estimate_asymptotic_complexity(inputs, times)
        } else {
            None
        };

        let now = Utc::now().naive_utc();
        let mut tx = pool.begin().await.unwrap();

        let submission: Submission = sqlx::query_as(
            r#"
            INSERT INTO submissions (
                problem_id,
                user_id,
                success,
                runtime,
                error,
                code,
                time,
                complexity
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(self.problem_id)
        .bind(self.user_id)
        .bind(passed)
        .bind(runtime)
        .bind(error)
        .bind(&self.implementation)
        .bind(now)
        .bind(complexity)
        .fetch_one(&mut tx)
        .await
        .map_err(|_| ServerError::InternalError)?;

        for test in &tests {
            let output = serde_json::to_string(&test.output).unwrap();

            sqlx::query!(
                r#"
                INSERT INTO test_results (
                    submission_id,
                    test_id,
                    runtime,
                    output,
                    error,
                    success
                ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
                submission.id,
                test.id,
                test.fuel,
                output,
                test.error,
                test.success,
            )
            .execute(&mut tx)
            .await
            .map_err(|_| ServerError::InternalError)?;
        }

        tx.commit().await.map_err(|_| ServerError::InternalError)?;

        // Broadcast the submission in the appropriate category
        if submission.success {
            // If exactly one person has
            let subs = sqlx::query!(
                "SELECT COUNT(id) as count FROM submissions WHERE problem_id = ? AND success = true",
                self.problem_id
            )
                .fetch_one(pool)
                .await
                .expect("Couldn't fetch row count");

            let message = if subs.count == 1 {
                BroadcastMessage::NewStar(submission.clone())
            } else {
                BroadcastMessage::NewCompletion(submission.clone())
            };

            broadcast.send(message).ok();
        }

        Ok(serde_json::to_value(submission).unwrap())
    }

    fn info(&self) -> String {
        format!(
            "SubmitJob for problem {} submitted by user {}",
            self.problem_id, self.user_id
        )
    }

    fn job_type(&self) -> String {
        "SubmitJob".to_string()
    }

    fn problem_id(&self) -> i64 {
        self.problem_id
    }
}
