//! The backend.

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::account::user_submissions;
use api::leaderboard::first_place_finishes;
use clap::Parser;
use reqwest::Client;

use api::{
    account::user_info,
    leaderboard::leaderboard,
    meetings::{edit_meeting, meeting, meeting_activities, meeting_list, next_meeting},
    problems::{create_problem, problem, problem_list, problem_tests},
    run::{custom_input, generate_tests, run_tests},
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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = State::new(&args.database_url).await;
    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(state.clone()))
            // TODO: Benchmark storing client in web::Data vs creating a new one per request
            .app_data(web::Data::new(client.clone()))
            .service(leaderboard)
            .service(first_place_finishes)
            .service(login)
            .service(signup)
            .service(problem_list)
            .service(problem)
            .service(problem_tests)
            .service(create_problem)
            .service(run_tests)
            .service(generate_tests)
            .service(custom_input)
            .service(user_info)
            .service(user_submissions)
            .service(meeting_list)
            .service(next_meeting)
            .service(meeting_activities)
            .service(meeting)
            .service(edit_meeting)
    })
    .bind(&format!("{}:{}", args.hostname, args.port))?
    .run()
    .await
}
