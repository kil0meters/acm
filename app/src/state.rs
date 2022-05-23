use acm::models::{
    forms::CreateProblemForm,
    runner::{RunnerError, RunnerResponse},
    Session,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yewdux::store::Store;

#[derive(Default, Deserialize, Serialize, PartialEq, Store, Clone)]
#[store(storage = "local")]
pub struct State {
    pub problem_editor: CreateProblemForm,
    pub session: Option<Session>,

    /// Associates test results with problem IDs
    pub test_results: HashMap<i64, Result<RunnerResponse, RunnerError>>,

    /// Stores the code for a problem
    pub problem_code: HashMap<i64, String>,

    /// Stores whether or not a the tests menu is shown
    pub tests_shown: bool,

    /// The currently displayed error by the program
    /// We don't want this to be saved
    #[serde(skip)]
    pub error: Option<String>,
}
