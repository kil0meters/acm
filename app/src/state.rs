use acm::models::{forms::CreateProblemForm, Session, runner::{RunnerResponse, RunnerError}};
use serde::{Deserialize, Serialize};
use yewdux::store::Store;
use std::collections::HashMap;

#[derive(Default, Deserialize, Serialize, PartialEq, Store, Clone)]
#[store(storage = "local")]
pub struct State {
    pub problem_editor: CreateProblemForm,
    pub session: Option<Session>,

    // Associates test results with problem IDs
    pub test_results: HashMap<i64, Result<RunnerResponse, RunnerError>>,

    // A
    pub problem_code: HashMap<i64, String>,
}
