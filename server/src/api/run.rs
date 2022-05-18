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
    Responder,
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
            // let tests = state.get_tests_for_problem(problem.id).await;

            let res = client
                .post(&format!("{RAMIEL_URL}/run/g++"))
                .json(&RunnerForm {
                    problem_id: problem.id,
                    runner_code: problem.runner,
                    test_code: form.test_code,
                    tests: vec![],
                })
                .send()
                .await
                // TODO: Handle error
                .unwrap();

            // TODO: Minor? inefficiency: simply pass through text rather than deserializing and
            // serializing
            if res.status().is_success() {
                let res = res.json::<RunnerResponse>().await.unwrap();
                api_success(res)
            } else {
                let res = res.json::<RunnerError>().await.unwrap();
                api_success(res)
            }
        }
        None => api_error(StatusCode::NOT_FOUND, "problem not found"),
    }
}
