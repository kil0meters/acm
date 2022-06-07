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
    problems::{create_problem, problem, problem_history, problem_list, problem_tests},
    run::{custom_input, generate_tests, submit_problem},
    signup::{login, signup},
    submissions::{submission, submission_tests},
};
use state::State;

mod api;
mod state;

pub type SqlPool = sqlx::SqlitePool;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 8081)]
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
    state.migrate().await;

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
            .service(problem_history)
            .service(create_problem)
            .service(submit_problem)
            .service(submission)
            .service(generate_tests)
            .service(custom_input)
            .service(user_info)
            .service(user_submissions)
            .service(meeting_list)
            .service(next_meeting)
            .service(meeting_activities)
            .service(submission_tests)
            .service(meeting)
            .service(edit_meeting)
    })
    .bind(&format!("{}:{}", args.hostname, args.port))?
    .run()
    .await
}
