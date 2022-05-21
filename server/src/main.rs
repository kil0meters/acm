//! The backend.

use acm::SERVER_URL;
use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{middleware::Logger, web, App, HttpServer};
use reqwest::Client;

use api::{
    leaderboard::leaderboard,
    problems::{create_problem, problem, problem_list},
    run::run_tests,
    signup::{login, signup},
};
use state::State;

mod api;
mod state;

pub type SqlPool = sqlx::SqlitePool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = State::new_state().await;
    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            // TODO: Benchmark storing client in web::Data vs creating a new one per request
            .app_data(web::Data::new(client.clone()))
            .service(
                // The entire api is scoped behind "/api/" to avoid collisions with regular pages.
                web::scope("/api")
                    .service(leaderboard)
                    .service(login)
                    .service(signup)
                    .service(problem_list)
                    .service(problem)
                    .service(create_problem)
                    .service(run_tests),
            )
            .service(
                // We serve from the dist directory generated by trunk, sending any directory not
                // already served by a file to index.html. Otherwise navigating to {site}/problems
                // wouldn't work properly for example.
                Files::new("/", "./dist/")
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("./dist/index.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })),
            )
    })
    .bind(SERVER_URL)?
    .run()
    .await
}
