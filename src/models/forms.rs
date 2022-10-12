use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::test::Test;

#[derive(Deserialize, Serialize)]
pub struct RunnerForm {
    pub problem_id: i64,
    pub user_id: i64,
    pub runner: String,
    pub implementation: String,
    pub tests: Vec<Test>,
}

#[derive(Deserialize, Serialize)]
pub struct GenerateTestsForm {
    pub runner: String,
    pub reference: String,
    pub user_id: i64,
    pub runtime_multiplier: f64,
    pub inputs: Vec<String>,
}

// TODO: Make naming less bad
#[derive(Deserialize, Serialize)]
pub struct RunnerCustomProblemInputForm {
    pub problem_id: i64,
    pub user_id: i64,
    pub runner: String,
    pub implementation: String,
    pub runtime_multiplier: f64,
    pub reference: String,
    pub input: String,
}

#[derive(Deserialize, Serialize)]
pub struct FirstTimeCompletionsForm {
    pub since: Option<NaiveDateTime>,
}
