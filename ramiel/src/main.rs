use acm::{
    models::{
        forms::RunnerCustomProblemInputForm,
        runner::{RunnerError, RunnerResponse},
        test::TestResult,
    },
    models::{
        forms::{GenerateTestsForm, RunnerForm},
        test::Test,
    },
};
use actix_web::{middleware::Logger, post, web, web::Json, App, HttpServer};

mod runners;

use clap::Parser;
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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, env, long, default_value_t = 8082)]
    port: u16,

    #[clap(short, long, env, default_value = "127.0.0.1")]
    hostname: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = Args::parse();

    let json_cfg = web::JsonConfig::default()
        // 3mb limit
        .limit(100_000_000);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(json_cfg.clone())
            .service(cplusplus_run)
            .service(cplusplus_generate_tests)
            .service(cplusplus_custom_input)
    })
    .bind(&format!("{}:{}", args.hostname, args.port))?
    .run()
    .await
}
