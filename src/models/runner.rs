use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

use crate::models::test::TestResult;

#[derive(Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct RunnerResponse {
    pub failed_tests: BTreeSet<TestResult>,
    pub passed_tests: BTreeSet<TestResult>,
}

impl RunnerResponse {
    pub fn new() -> Self {
        Self {
            failed_tests: BTreeSet::new(),
            passed_tests: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, test: TestResult) {
        if test.output == test.expected_output {
            self.passed_tests.insert(test);
        } else {
            self.failed_tests.insert(test);
        }
    }
}

#[derive(Deserialize, Serialize, Error, Debug, Clone, PartialEq)]
pub enum RunnerError {
    #[error("line {line}:\n{message}")]
    CompilationError { line: u64, message: String },

    #[error("encountered a runtime error")]
    RuntimeError(String),
}
