use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{get, middleware::Logger, web, web::Json, App, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use state::State;

use api::{
    leaderboard::leaderboard,
    problems::{create_problem, problem, problem_list},
    signup::{login, signup},
};

mod api;
mod state;

pub type SqlPool = sqlx::SqlitePool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = State::new_state().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/api")
                    .service(leaderboard)
                    .service(login)
                    .service(signup)
                    .service(problem_list)
                    .service(problem)
                    .service(
                        web::scope("/authorized")
                            .wrap(HttpAuthentication::bearer(state::auth::validator))
                            .service(create_problem),
                    ),
            )
            .service(
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
