use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize, Debug, Default, Serialize, Clone, PartialEq)]
pub struct Test {
    #[serde(default)]
    pub id: i64,
    pub index: i64,
    pub input: String,
    pub expected_output: String,
}

impl Test {
    pub fn make_result(self, output: String, time: Duration) -> TestResult {
        TestResult {
            id: self.id,
            index: self.index,
            input: self.input,
            expected_output: self.expected_output,
            output,

            // TODO: This breaks if any test takes longer than 1 second. Realistically this should
            // not be a big deal.
            time: time.subsec_nanos().into(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct TestResult {
    #[serde(default)]
    pub id: i64,
    pub index: i64,
    pub input: String,
    pub expected_output: String,
    pub output: String,
    pub time: i64,
}
