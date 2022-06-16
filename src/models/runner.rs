use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::test::TestResult;

#[derive(Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct RunnerResponse {
    pub tests: Vec<TestResult>,
    pub passed: bool,

    // runtime, stored as milliseconds
    pub runtime: i64,
}

#[derive(Deserialize, Serialize, Error, Debug, Clone, PartialEq)]
#[serde(tag="type")]
pub enum RunnerError {
    #[error("{message}")]
    CompilationError { line: u64, message: String },

    #[error("encountered a runtime error")]
    RuntimeError { message: String },

    #[error("Encountered an error while running code: {}", message)]
    InternalServerError { message: String },

    #[error("Process took too long to execute")]
    TimeoutError { message: String },
}

impl From<std::io::Error> for RunnerError {
    fn from(e: std::io::Error) -> Self {
        RunnerError::InternalServerError { message: e.to_string() }
    }
}
