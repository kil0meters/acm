use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

use crate::models::test::TestResult;

#[derive(Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct RunnerResponse {
    pub failed_tests: BTreeSet<TestResult>,
    pub passed_tests: BTreeSet<TestResult>,

    // runtime, stored as milliseconds
    pub runtime: i64,
}

impl RunnerResponse {
    pub fn new() -> Self {
        Self {
            failed_tests: BTreeSet::new(),
            passed_tests: BTreeSet::new(),
            runtime: 0,
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

    #[error("Encountered an error while running code: {}", .0)]
    InternalServerError(String),
}

impl From<std::io::Error> for RunnerError {
    fn from(e: std::io::Error) -> Self {
        RunnerError::InternalServerError(e.to_string())
    }
}
