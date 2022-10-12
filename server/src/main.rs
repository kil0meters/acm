use std::{
    collections::HashMap,
    net::SocketAddr,
    process::exit,
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};
use tokio::sync::{broadcast, mpsc, RwLock};

use axum::{routing::get, Extension, Router, Server};
use clap::Parser;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    problems::Problem,
    run::{job_worker, JobQueueItem, JobStatus},
    ws::BroadcastMessage,
};

mod auth;
mod error;
mod leaderboard;
mod meetings;
mod problems;
mod run;
mod submissions;
mod user;
mod ws;

pub const MAX_TEST_LENGTH: usize = 500;

pub static JOB_COUNTER: AtomicU64 = AtomicU64::new(0);
pub static PROCESSING_JOB: AtomicU64 = AtomicU64::new(0);

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, env, long, default_value_t = 8081)]
    port: u16,

    #[clap(short, long, env, default_value = "127.0.0.1")]
    hostname: String,

    #[clap(long, env, default_value = "./db.sqlite")]
    database_url: String,

    #[clap(long, env, default_value = "http://127.0.0.1:8082")]
    ramiel_url: String,

    #[clap(env)]
    jwt_secret: String,

    #[clap(env)]
    discord_secret: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=debug,sqlx=warn")
        .init();

    // A broadcast channel to update new submissions in real time.
    let (broadcast, _) = broadcast::channel::<BroadcastMessage>(16);

    // A multi-producer, single-consumer channel for long-running jobs
    let (job_queue, rx) = mpsc::channel::<(u64, JobQueueItem)>(10);
    let queued_jobs = Arc::new(RwLock::new(HashMap::<u64, JobStatus>::new()));

    tracing::info!("Connecting to database at \"{}\"", args.database_url);
    let pool = match SqlitePool::connect(&args.database_url).await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("error {e}");
            exit(1);
        }
    };

    // Spawn job queue thread
    {
        let ramiel_url = args.ramiel_url.clone();
        let queued_jobs = queued_jobs.clone();
        let broadcast = broadcast.clone();
        let pool = pool.clone();
        tokio::spawn(async move {
            job_worker(rx, queued_jobs, ramiel_url, pool, broadcast).await;
        });
    }

    // Spawn problem publish notification thread
    {
        let pool = pool.clone();
        let broadcast = broadcast.clone();
        tokio::spawn(async move {
            loop {
                log::info!("Checking for problems to be made visible");

                let rows = sqlx::query_as!(
                    Problem,
                    r#"
                    SELECT
                        id,
                        title,
                        description,
                        runner,
                        template
                    FROM
                        problems
                    WHERE
                        visible = false AND publish_time < datetime('now')
                "#
                )
                .fetch_all(&pool)
                .await
                .unwrap();

                for problem in rows {
                    sqlx::query!(
                        r#"
                    UPDATE
                        problems
                    SET
                        visible = true
                    WHERE
                        id = ?
                    "#,
                        problem.id
                    )
                    .execute(&pool)
                    .await
                    .unwrap();

                    broadcast.send(BroadcastMessage::NewProblem(problem)).ok();
                }

                tokio::time::sleep(Duration::new(30, 0)).await;
            }
        });
    }

    if let Err(e) = sqlx::migrate!("../migrations").run(&pool).await {
        log::error!("Migration error: {e:?}");
        exit(1);
    }

    let addr = SocketAddr::new(args.hostname.parse().unwrap(), args.port);
    tracing::info!("Started server on {addr}");

    let app = Router::new()
        .nest("/auth", auth::routes())
        .nest("/user", user::routes())
        .nest("/leaderboard", leaderboard::routes())
        .nest("/submissions", submissions::routes())
        .nest("/run", run::routes())
        .nest("/problems", problems::routes())
        .nest("/meetings", meetings::routes())
        .route("/ws", get(ws::handler))
        .layer(Extension(args.ramiel_url))
        .layer(Extension(queued_jobs))
        .layer(Extension(pool))
        .layer(Extension(broadcast))
        .layer(Extension(job_queue))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive());

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
