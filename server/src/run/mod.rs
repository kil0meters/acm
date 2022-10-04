use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc},
};

use axum::{
    async_trait,
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Serialize;
use serde_json::Value;
use sqlx::SqlitePool;
use tokio::{
    sync::{broadcast, mpsc, RwLock},
    time::{sleep, Duration},
};

use crate::{
    auth::Claims,
    error::{AuthError, ServerError},
    ws::BroadcastMessage,
    JOB_COUNTER, PROCESSING_JOB,
};

mod custom;
mod generate_tests;
mod submit;

pub type JobQueueItem = Box<dyn Queueable + Send + Sync>;
pub type JobQueue = mpsc::Sender<(u64, JobQueueItem)>;
pub type JobMap = Arc<RwLock<HashMap<u64, JobStatus>>>;

#[derive(Serialize, Clone)]
pub struct JobStatus {
    id: u64,

    #[serde(skip)]
    user_id: i64,

    queue_position: u64,

    response: Option<Value>,
    error: Option<Value>,
}

#[async_trait]
pub trait Queueable {
    // Executes the job
    async fn run(
        &self,
        ramiel_url: &str,
        pool: &SqlitePool,
        broadcast: &broadcast::Sender<BroadcastMessage>,
    ) -> Result<Value, ServerError>;

    // Returns some basic info about the job -- for logging purposes only
    fn info(&self) -> String;
}

// Adds a job to the job queue
async fn add_job(
    user_id: i64,
    job_queue: JobQueue,
    job_map: JobMap,
    queue_item: JobQueueItem,
) -> Result<JobStatus, ServerError> {
    let job_id = JOB_COUNTER.fetch_add(1, Ordering::SeqCst);

    log::info!("Adding job {job_id}: {}", queue_item.info());

    let job_status = JobStatus {
        id: job_id,
        user_id,

        queue_position: job_id - PROCESSING_JOB.load(Ordering::SeqCst),

        response: None,
        error: None,
    };

    job_map.write().await.insert(job_id, job_status.clone());

    job_queue
        .send((job_id, queue_item))
        .await
        .map_err(|_| ServerError::InternalError)?;

    Ok(job_status)
}

pub async fn check_job(
    Path(id): Path<u64>,
    Extension(job_map): Extension<JobMap>,
    claims: Claims,
) -> Result<Json<JobStatus>, ServerError> {
    if let Some(job) = job_map.read().await.get(&id) {
        if job.user_id == claims.user_id {
            let processing_job = PROCESSING_JOB.load(Ordering::SeqCst);
            let mut job = job.clone();
            if job.id >= processing_job {
                job.queue_position = job.id - processing_job;
            }
            Ok(Json(job.to_owned()))
        } else {
            Err(AuthError::Unauthorized.into())
        }
    } else {
        Err(ServerError::NotFound)
    }
}

pub async fn job_worker(
    mut rx: mpsc::Receiver<(u64, JobQueueItem)>,
    queued_jobs: JobMap,
    ramiel_url: String,
    pool: SqlitePool,
    broadcast: broadcast::Sender<BroadcastMessage>,
) {
    log::info!("Started job worker");

    while let Some((id, queue_item)) = rx.recv().await {
        PROCESSING_JOB.store(id, Ordering::SeqCst);

        log::info!("Processing job {id}: {}", queue_item.info());

        let res = queue_item.run(&ramiel_url, &pool, &broadcast).await;

        let mut job_map_writer = queued_jobs.write().await;
        let job = job_map_writer.get_mut(&id).expect("Job missing in job map");

        match res {
            Ok(res) => {
                job.response = Some(res);
            }
            Err(e) => {
                let body = serde_json::to_value(e).unwrap();
                job.error = Some(body);
            }
        }

        let job_map = queued_jobs.clone();
        // Set timeout to remove the job from the job map to prevent it from growing out of control
        tokio::spawn(async move {
            sleep(Duration::from_secs(10)).await;

            job_map.write().await.remove(&id);
            log::info!("Job {id} purged from job map");
        });
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/custom", post(custom::custom))
        .route("/generate-tests", post(generate_tests::generate_tests))
        .route("/submit", post(submit::submit))
        .route("/check/:id", get(check_job))
}
