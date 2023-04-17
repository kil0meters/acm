use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::test::TestResult;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DiagnosticType {
    Error,
    Warning,
    Note,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub line: usize,
    pub col: usize,
    pub diagnostic_type: DiagnosticType,
    pub message: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct RunnerResponse {
    pub tests: Vec<TestResult>,
    pub passed: bool,

    // runtime, stored as milliseconds
    pub runtime: i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CustomInputResponse {
    pub result: TestResult,
    pub output: String,
}

#[derive(Deserialize, Serialize, Error, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum RunnerError {
    #[error("{diagnostics:?}")]
    CompilationError { diagnostics: Vec<Diagnostic> },

    #[error("encountered a runtime error:\n{message}")]
    RuntimeError { message: String },

    #[error("Internal error:\n{}", message)]
    InternalServerError { message: String },

    #[error("Process took too long to execute")]
    TimeoutError { message: String },
}

impl From<std::io::Error> for RunnerError {
    fn from(e: std::io::Error) -> Self {
        RunnerError::InternalServerError {
            message: e.to_string(),
        }
    }
}
