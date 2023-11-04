use std::time::Duration;

use shared::models::{
    forms::{CustomInputJob, GenerateTestsJob, SubmitJob},
    runner::{CustomInputResponse, RunnerError, RunnerResponse},
    test::Test,
};

use actix_web::{middleware::Logger, post, web, web::Json, App, HttpServer};

mod runners;

use clap::Parser;
use runners::{CPlusPlus, Runner};

#[post("/run/c++")]
async fn cplusplus_run(form: Json<SubmitJob>) -> Json<Result<RunnerResponse, RunnerError>> {
    let task = tokio::spawn(async {
        let res = Json(CPlusPlus.run_tests(form.into_inner()).await);
        log::info!("The task wasn't cancelled!!!!!");
        res
    });

    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(90)) => {
            return Json(Err(RunnerError::TimeoutError { message: "The tests took too long to run. (process killed)".to_string() }));
        }
        res = task => {
            return res.unwrap();
        }
    }
}

#[post("/generate-tests/c++")]
async fn cplusplus_generate_tests(
    form: Json<GenerateTestsJob>,
) -> Json<Result<Vec<Test>, RunnerError>> {
    let task = tokio::spawn(async {
        let res = Json(CPlusPlus.generate_tests(form.into_inner()).await);
        log::info!("The task wasn't cancelled!!!!!");
        res
    });

    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(120)) => {
            return Json(Err(RunnerError::TimeoutError { message: "The tests took too long to run. (process killed)".to_string() }));
        }
        res = task => {
            return res.unwrap();
        }
    }
}

#[post("/custom-input/c++")]
async fn cplusplus_custom_input(
    form: Json<CustomInputJob>,
) -> Json<Result<CustomInputResponse, RunnerError>> {
    let task = tokio::spawn(async {
        let res = Json(CPlusPlus.run_custom_input(form.into_inner()).await);
        log::info!("The task wasn't cancelled!!!!!");
        res
    });

    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(60)) => {
            return Json(Err(RunnerError::TimeoutError { message: "The tests took too long to run. (process killed)".to_string() }));
        }
        res = task => {
            return res.unwrap();
        }
    }
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));
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
