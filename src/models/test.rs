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
            success: output == self.expected_output,
            input: self.input,
            expected_output: self.expected_output,
            output,

            // TODO: This breaks if any test takes longer than 1 second. Realistically this should
            // not be a big deal.
            runtime: time.subsec_nanos().into(),
        }
    }

    pub fn truncate(&mut self, size: usize) {
        if self.input.len() > size {
            self.input.truncate(size);
            self.input.push_str("\n...");
        }

        if self.expected_output.len() > size {
            self.expected_output.truncate(size);
            self.expected_output.push_str("\n...");
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct TestResult {
    #[serde(default)]
    pub id: i64,
    pub index: i64,
    pub success: bool,
    pub input: String,
    pub expected_output: String,
    pub output: String,
    pub runtime: i64,
}

impl TestResult {
    pub fn truncate(&mut self, size: usize) {
        if self.input.len() > size {
            self.input.truncate(size);
            self.input.push_str("\n...");
        }

        if self.expected_output.len() > size {
            self.expected_output.truncate(size);
            self.expected_output.push_str("\n...");
        }

        if self.output.len() > size {
            self.output.truncate(size);
            self.output.push_str("\n...");
        }
    }
}
