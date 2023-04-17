use std::{
    collections::{HashMap, VecDeque},
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
use shared::models::runner::RunnerError;
use sqlx::SqlitePool;
use tokio::{
    sync::{
        broadcast::{self, Sender},
        mpsc, RwLock,
    },
    task::JoinHandle,
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

pub type JobQueueItem = Box<dyn Queueable>;
pub type JobQueue = mpsc::UnboundedSender<(u64, JobQueueItem)>;
pub type JobMap = Arc<RwLock<HashMap<u64, JobStatus>>>;

#[derive(Serialize, Debug, Clone)]
pub struct JobStatus {
    id: u64,

    user_id: i64,

    queue_position: u64,
    job_type: String,
    problem_id: i64,

    response: Option<Value>,
    error: Option<String>,
}

#[async_trait]
pub trait Queueable: Send + Sync {
    // Executes the job
    async fn run(
        &self,
        ramiel_url: &str,
        pool: &SqlitePool,
        broadcast: &broadcast::Sender<BroadcastMessage>,
    ) -> Result<Value, ServerError>;

    // Returns some basic info about the job -- for logging purposes only
    fn info(&self) -> String;
    fn job_type(&self) -> String;
    fn problem_id(&self) -> i64;
}

// Adds a job to the job queue
async fn add_job(
    user_id: i64,
    job_queue: JobQueue,
    job_map: JobMap,
    queue_item: JobQueueItem,
    broadcast: Sender<BroadcastMessage>,
) -> Result<JobStatus, ServerError> {
    let job_id = JOB_COUNTER.fetch_add(1, Ordering::SeqCst);

    log::info!("Adding job {job_id}: {}", queue_item.info());

    let job_status = JobStatus {
        id: job_id,
        user_id,

        queue_position: job_id - PROCESSING_JOB.load(Ordering::SeqCst),
        job_type: queue_item.job_type(),
        problem_id: queue_item.problem_id(),
        response: None,
        error: None,
    };

    broadcast
        .send(BroadcastMessage::NewJob(job_status.clone()))
        .ok();

    job_map.write().await.insert(job_id, job_status.clone());

    job_queue
        .send((job_id, queue_item))
        .map_err(|_| ServerError::InternalError)?;

    Ok(job_status)
}

pub async fn check_job(
    Path(id): Path<u64>,
    Extension(job_map): Extension<JobMap>,
    claims: Claims,
) -> Result<Json<JobStatus>, ServerError> {
    claims.validate_logged_in()?;

    if let Some(job) = job_map.read().await.get(&id) {
        if job.user_id == claims.user_id {
            let processing_job = PROCESSING_JOB.load(Ordering::SeqCst);
            let mut job = job.clone();
            if job.id >= processing_job {
                job.queue_position = job.id - processing_job;
            }
            Ok(Json(job))
        } else {
            Err(AuthError::Unauthorized.into())
        }
    } else {
        Err(ServerError::NotFound)
    }
}

async fn process_job(
    id: u64,
    queue_item: JobQueueItem,
    queued_jobs: JobMap,
    ramiel_url: String,
    pool: SqlitePool,
    broadcast: broadcast::Sender<BroadcastMessage>,
) {
    PROCESSING_JOB.store(id, Ordering::SeqCst);

    log::info!("Processing job {id}: {}", queue_item.info());

    let res = queue_item.run(&ramiel_url, &pool, &broadcast).await;

    let mut job_map_writer = queued_jobs.write().await;
    let job = job_map_writer.get_mut(&id).expect("Job missing in job map");

    log::info!("{res:?}");

    match res {
        Ok(res) => {
            job.response = Some(res);
        }
        Err(e) => job.error = Some(e.to_string()),
    }

    broadcast
        .send(BroadcastMessage::FinishedJob(job.clone()))
        .ok();

    let job_map = queued_jobs.clone();
    // Set timeout to remove the job from the job map to prevent it from growing out of control
    tokio::spawn(async move {
        sleep(Duration::from_secs(10)).await;

        job_map.write().await.remove(&id);
        log::info!("Job {id} purged from job map");
    });
}

pub async fn job_worker(
    mut rx: mpsc::UnboundedReceiver<(u64, JobQueueItem)>,
    queued_jobs: JobMap,
    ramiel_url: String,
    pool: SqlitePool,
    broadcast: broadcast::Sender<BroadcastMessage>,
    parallel_job_count: u8,
) {
    log::info!("Started job worker");

    let mut tasks: VecDeque<JoinHandle<()>> = VecDeque::with_capacity(parallel_job_count.into());

    while let Some((id, queue_item)) = rx.recv().await {
        if tasks.len() >= parallel_job_count.into() {
            if let Some(task) = tasks.pop_front() {
                task.await.unwrap();
            }
        }

        let ramiel_url = ramiel_url.clone();
        let queued_jobs = queued_jobs.clone();
        let broadcast = broadcast.clone();
        let pool = pool.clone();
        tasks.push_back(tokio::spawn(async move {
            process_job(id, queue_item, queued_jobs, ramiel_url, pool, broadcast).await;
        }));
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/custom", post(custom::custom))
        .route("/generate-tests", post(generate_tests::generate_tests))
        .route("/submit", post(submit::submit))
        .route("/check/:id", get(check_job))
}
