use acm::{
    models::{
        forms::{RunTestsForm, RunnerForm, GenerateTestsForm, RunnerCustomProblemInputForm, CustomProblemInputForm},
        runner::{RunnerError, RunnerResponse},
        Auth, test::{Test, TestResult}
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
pub async fn run_tests( form: Json<RunTestsForm>,
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
                .post(&format!("http://{RAMIEL_URL}/run/c++"))
                .json(&RunnerForm {
                    problem_id: problem.id,
                    username: claims.username.clone(),
                    runner: problem.runner,
                    implementation: form.test_code.clone(),
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

#[post("/generate-tests")]
pub async fn generate_tests(
    form: Json<GenerateTestsForm>,
    client: Data<Client>,
    claims: Claims,
) -> impl Responder {
    let form = form.into_inner();
    let client = client.into_inner();

    match claims.auth {
        Auth::ADMIN | Auth::OFFICER => {
            let res = client
                .post(&format!("http://{RAMIEL_URL}/generate-tests/c++"))
                .json(&form)
                .send()
                .await
                // TODO: Handle error
                .unwrap();

            let tests: Result<Vec<Test>, RunnerError> = res.json().await.unwrap();
            api_success(tests)
        },
        Auth::MEMBER => api_error(
            StatusCode::UNAUTHORIZED,
            "You must be an officer to do that",
        ),
    }
}

#[post("/custom-input")]
pub async fn custom_input(
    form: Json<CustomProblemInputForm>,
    client: Data<Client>,
    state: AppState,
    claims: Claims,
) -> impl Responder {
    let form = form.into_inner();

    if let Some (problem) = state.problems_get_by_id(form.problem_id).await {
        let res = client
            .post(&format!("http://{RAMIEL_URL}/custom-input/c++"))
            .json(&RunnerCustomProblemInputForm {
                problem_id: problem.id,
                runner: problem.runner,
                username: claims.username,
                implementation: form.implementation,
                reference: problem.reference,
                input: form.input
            })
            .send()
            .await
            // TODO: Handle error
            .unwrap();

        let result: Result<TestResult, RunnerError> = res.json().await.unwrap();

        api_success(result)
    } else {
        api_error(StatusCode::NOT_FOUND, "Problem not found")
    }
}
