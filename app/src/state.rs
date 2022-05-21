use acm::models::test::Test;
use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Default, Deserialize, Serialize, PartialEq, Store, Clone)]
#[store(storage = "local")]
pub struct State {
    pub tests: Vec<Test>,
}
