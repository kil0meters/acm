use acm::{models::forms::RunnerForm, RAMIEL_URL};
use actix_web::{
    middleware::Logger,
    post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};

mod runners;

use runners::{GPlusPlus, Runner};

async fn handle_runner(runner: impl Runner, form: RunnerForm) -> impl Responder {
    let res = runner
        .run_tests(
            form.problem_id,
            &form.runner_code,
            &form.test_code,
            form.tests,
        )
        .await;

    match res {
        Ok(res) => HttpResponse::Ok().json(&res),
        Err(e) => HttpResponse::Ok().json(&e),
    }
}

#[post("/run/g++")]
async fn gplusplus(form: Json<RunnerForm>) -> impl Responder {
    handle_runner(GPlusPlus::new(), form.into_inner()).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(gplusplus)
    })
    .bind(RAMIEL_URL)?
    .run()
    .await
}
