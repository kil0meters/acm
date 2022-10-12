use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize, Clone, PartialEq)]
pub struct Test {
    #[serde(default)]
    pub id: i64,
    pub index: i64,
    pub input: String,
    pub max_runtime: Option<i64>,
    pub expected_output: String,
}

impl Test {
    pub fn make_result(self, output: String, fuel: u64) -> TestResult {
        TestResult {
            id: self.id,
            index: self.index,
            success: output == self.expected_output,
            input: self.input,
            expected_output: self.expected_output,
            output,
            error: None,
            max_runtime: self.max_runtime,
            runtime: fuel as i64,
        }
    }

    pub fn make_result_error(self, error: String, runtime: i64) -> TestResult {
        TestResult {
            id: self.id,
            index: self.index,
            success: false,
            input: self.input,
            expected_output: self.expected_output,
            output: String::new(),
            error: Some(error),
            max_runtime: self.max_runtime,
            runtime,
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
    pub error: Option<String>,
    pub max_runtime: Option<i64>,
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
