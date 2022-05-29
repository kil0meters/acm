//! The backend.

use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::account::user_submissions;
use api::leaderboard::first_place_finishes;
use clap::Parser;
use reqwest::Client;

use api::{
    account::user_info,
    leaderboard::leaderboard,
    meetings::{edit_meeting, meeting, meeting_list, next_meeting},
    problems::{create_problem, problem, problem_list, problem_tests},
    run::run_tests,
    signup::{login, signup},
};
use state::State;

mod api;
mod state;

pub type SqlPool = sqlx::SqlitePool;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 8080)]
    port: u16,

    #[clap(short, long, default_value = "127.0.0.1")]
    hostname: String,

    #[clap(long, default_value = "./db.sqlite")]
    database_url: String,

    #[clap(long, default_value = "./dist")]
    dist_path: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = State::new(&args.database_url).await;
    let client = Client::new();

    HttpServer::new(move || {
        let index_path = format!("{}/index.html", args.dist_path);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            // TODO: Benchmark storing client in web::Data vs creating a new one per request
            .app_data(web::Data::new(client.clone()))
            .service(
                // The entire api is scoped behind "/api/" to avoid collisions with regular pages.
                web::scope("/api")
                    .service(leaderboard)
                    .service(first_place_finishes)
                    .service(login)
                    .service(signup)
                    .service(problem_list)
                    .service(problem)
                    .service(problem_tests)
                    .service(create_problem)
                    .service(run_tests)
                    .service(user_info)
                    .service(user_submissions)
                    .service(meeting_list)
                    .service(next_meeting)
                    .service(meeting)
                    .service(edit_meeting),
            )
            .service(Files::new("/static/", &args.dist_path))
            .default_service(fn_service(move |req: ServiceRequest| {
                let index_path = index_path.clone();
                async move {
                    let (req, _) = req.into_parts();
                    let file = NamedFile::open_async(&index_path).await?;
                    let res = file.into_response(&req);
                    Ok(ServiceResponse::new(req, res))
                }
            }))
    })
    .bind(&format!("{}:{}", args.hostname, args.port))?
    .run()
    .await
}
