use acm::models::{forms::GenerateTestsForm, runner::RunnerError, test::Test};
use axum::{Extension, Json};
use reqwest::Client;

use crate::{auth::Claims, error::ServerError};

pub async fn generate_tests(
    claims: Claims,
    Extension(ramiel_url): Extension<String>,
    Json(form): Json<GenerateTestsForm>,
) -> Result<Json<Vec<Test>>, ServerError> {
    claims.validate_officer()?;

    let client = Client::new();
    let res = client
        .post(&format!("{ramiel_url}/generate-tests/c++"))
        .json(&form)
        .send()
        .await
        // TODO: Handle error
        .unwrap();

    let tests: Result<Vec<Test>, RunnerError> = res.json().await.unwrap();

    Ok(Json(tests?))
}
