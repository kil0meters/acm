use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize, Serialize)]
pub struct Test {
    pub index: i64,
    pub input: String,
    pub expected_output: String,
}

impl Test {
    pub fn make_result(self, output: String, time: Duration) -> TestResult {
        TestResult {
            index: self.index,
            input: self.input,
            expected_output: self.expected_output,
            output,
            time,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestResult {
    pub index: i64,
    pub input: String,
    pub expected_output: String,
    pub output: String,
    pub time: Duration,
}
