//! The backend.

use std::{net::SocketAddr, process::exit};

use axum::{Extension, Router, Server};
use clap::Parser;
use sqlx::SqlitePool;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

mod auth;
mod error;
mod leaderboard;
mod meetings;
mod problems;
mod run;
mod submissions;
mod user;

pub const MAX_TEST_LENGTH: usize = 500;

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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=debug,sqlx=warn")
        .init();

    tracing::info!("Connecting to database at \"{}\"", args.database_url);
    let pool = match SqlitePool::connect(&args.database_url).await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("{e}");
            exit(1);
        }
    };

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
        .layer(Extension(args.ramiel_url))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive());

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
