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
    state::{auth::Claims, AppState},
};

#[post("/run-tests")]
pub async fn run_tests(
    form: Json<RunTestsForm>,
    state: AppState,
    client: Data<Client>,
    claims: Claims,
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
                    test_code: form.test_code.clone(),
                    tests,
                })
                .send()
                .await
                // TODO: Handle error
                .unwrap();

            let res: Result<RunnerResponse, RunnerError> = res.json().await.unwrap();

            // Saves the result in the database
            // TODO: Handle errors
            state
                .save_submission(&res, &form.test_code, &claims.username, problem.id)
                .await
                .unwrap();

            api_success(res)
        }
        None => api_error(StatusCode::NOT_FOUND, "problem not found"),
    }
}
