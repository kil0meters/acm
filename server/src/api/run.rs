use acm::{
    models::{
        forms::{RunTestsForm, RunnerForm},
        runner::{RunnerError, RunnerResponse},
    },
    RAMIEL_URL,
};
use actix_web::{
    http::StatusCode,
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use reqwest::Client;

use crate::{
    api::{api_error, api_success},
    state::AppState,
};

#[post("/run-tests")]
pub async fn run_tests(
    form: Json<RunTestsForm>,
    state: AppState,
    client: Data<Client>,
) -> impl Responder {
    let form = form.into_inner();
    let client = client.into_inner();

    match state.problems_get_by_id(form.problem_id).await {
        Some(problem) => {
            let tests = state.tests_get_for_problem_id(problem.id).await;

            let res = client
                .post(&format!("http://{RAMIEL_URL}/run/g++"))
                .json(&RunnerForm {
                    problem_id: problem.id,
                    runner_code: problem.runner,
                    test_code: form.test_code,
                    tests,
                })
                .send()
                .await
                // TODO: Handle error
                .unwrap();

            HttpResponse::Ok()
                .content_type("application/json")
                .body(res.text().await.unwrap_or_default())
        }
        None => api_error(StatusCode::NOT_FOUND, "problem not found"),
    }
}
