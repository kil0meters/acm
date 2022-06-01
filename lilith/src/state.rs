use acm::models::{
    forms::{CreateProblemForm, EditMeetingForm},
    runner::{RunnerError, RunnerResponse},
    test::TestResult,
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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub problems: HashMap<i64, ProblemState>,

    /// Stores whether or not a the tests menu is shown
    pub tests_shown: bool,

    /// Stores in-progress editing
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub meeting_editor: HashMap<i64, EditMeetingForm>,

    /// The currently displayed error by the program
    /// We don't want this to be saved
    #[serde(skip)]
    pub error: Option<String>,
}

#[derive(Deserialize, Default, Serialize, PartialEq, Clone)]
pub struct ProblemState {
    pub implementation: String,
    pub custom_input: String,
    pub docker_shown: bool,
    pub custom_test_result: Option<TestResult>,
}
