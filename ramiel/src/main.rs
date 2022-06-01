use acm::{
    models::{
        forms::RunnerCustomProblemInputForm,
        runner::{RunnerError, RunnerResponse}, test::TestResult,
    },
    models::{
        forms::{GenerateTestsForm, RunnerForm},
        test::Test,
    },
    RAMIEL_URL,
};
use actix_web::{middleware::Logger, post, web::Json, App, HttpServer};

mod runners;

use runners::{CPlusPlus, Runner};

#[post("/run/c++")]
async fn cplusplus_run(form: Json<RunnerForm>) -> Json<Result<RunnerResponse, RunnerError>> {
    Json(CPlusPlus.run_tests(form.into_inner()).await)
}

#[post("/generate-tests/c++")]
async fn cplusplus_generate_tests(
    form: Json<GenerateTestsForm>,
) -> Json<Result<Vec<Test>, RunnerError>> {
    Json(CPlusPlus.generate_tests(form.into_inner()).await)
}

#[post("/custom-input/c++")]
async fn cplusplus_custom_input(
    form: Json<RunnerCustomProblemInputForm>,
) -> Json<Result<TestResult, RunnerError>> {
    Json(CPlusPlus.run_custom_input(form.into_inner()).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(cplusplus_run)
            .service(cplusplus_generate_tests)
            .service(cplusplus_custom_input)
    })
    .bind(RAMIEL_URL)?
    .run()
    .await
}
