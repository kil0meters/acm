use acm::models::{
    runner::{RunnerError, RunnerResponse},
    test::Test,
};
use axum::{async_trait, Extension, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::{auth::Claims, error::ServerError, submissions::Submission, ws::BroadcastMessage};

use super::{add_job, JobMap, JobQueue, JobStatus, Queueable};

#[derive(Deserialize)]
pub struct SubmitForm {
    pub problem_id: i64,
    pub implementation: String,
}

pub async fn submit(
    Json(form): Json<SubmitForm>,
    Extension(pool): Extension<SqlitePool>,
    Extension(job_queue): Extension<JobQueue>,
    Extension(job_map): Extension<JobMap>,
    claims: Claims,
) -> Result<Json<JobStatus>, ServerError> {
    claims.validate_logged_in()?;

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
            max_runtime,
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

    let queue_item = Box::new(SubmitJob {
        problem_id: form.problem_id,
        user_id: claims.user_id,
        runner,
        implementation: form.implementation.clone(),
        tests,
    });

    let job = add_job(claims.user_id, job_queue, job_map, queue_item).await?;

    Ok(Json(job))
}

#[derive(Serialize)]
pub struct SubmitJob {
    pub problem_id: i64,
    pub user_id: i64,
    pub runner: String,
    pub implementation: String,
    pub tests: Vec<Test>,
}

#[async_trait]
impl Queueable for SubmitJob {
    async fn run(
        &self,
        ramiel_url: &str,
        pool: &SqlitePool,
        broadcast: &broadcast::Sender<BroadcastMessage>,
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
            self.problem_id,
            self.user_id,
            passed,
            runtime,
            error,
            self.implementation
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
                error,
                success
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
                submission.id,
                test.id,
                test.runtime,
                test.output,
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
}
