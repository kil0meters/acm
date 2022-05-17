use actix_web::{
    get, post,
    web::{self, Json},
    App, HttpServer, Responder, HttpResponse,
};
use serde::{Deserialize, Serialize};

mod runners;

use runners::{GPlusPlus, Runner, RunnerError, Test};

#[derive(Deserialize, Serialize)]
struct RunnerForm {
    project_name: String,
    runner_code: String,
    test_code: String,
    tests: Vec<Test>,
}

async fn handle_runner(runner: impl Runner, form: RunnerForm) -> impl Responder {
    let res = runner
        .run_tests(
            &form.project_name,
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
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(gplusplus)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
