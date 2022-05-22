use acm::models::{forms::CreateProblemForm, test::Test, Session};
use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Default, Deserialize, Serialize, PartialEq, Store, Clone)]
#[store(storage = "local")]
pub struct State {
    pub problem_editor: CreateProblemForm,
    pub session: Option<Session>,
}
