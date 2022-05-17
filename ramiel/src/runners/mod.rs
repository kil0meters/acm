use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    io::Write,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use thiserror::Error;

mod gplusplus;

pub use gplusplus::GPlusPlus;

#[async_trait]
pub trait Runner {
    async fn run_tests(
        &self,
        project_name: &str,
        runner_code: &str,
        test_code: &str,
        tests: Vec<Test>,
    ) -> Result<RunnerResponse, RunnerError>;
}

#[derive(Deserialize, Serialize)]
pub struct RunnerResponse {
    failed_tests: BTreeSet<TestResult>,
    passed_tests: BTreeSet<TestResult>,
}

impl RunnerResponse {
    fn new() -> Self {
        Self {
            failed_tests: BTreeSet::new(),
            passed_tests: BTreeSet::new(),
        }
    }

    fn insert(&mut self, test: TestResult) {
        if test.output == test.expected_output {
            self.passed_tests.insert(test);
        } else {
            self.failed_tests.insert(test);
        }
    }
}

#[derive(Deserialize, Serialize, Error, Debug)]
pub enum RunnerError {
    #[error("line {line}:\n{message}")]
    CompilationError { line: u64, message: String },

    #[error("encountered a runtime error")]
    RuntimeError(String),
}

#[derive(Deserialize, Serialize)]
pub struct Test {
    index: u64,
    input: String,
    expected_output: String,
}

impl Test {
    fn make_result(self, output: String, time: Duration) -> TestResult {
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
    index: u64,
    input: String,
    expected_output: String,
    output: String,
    time: Duration,
}

impl TestResult {}

/// Runs a command with a specified input, returning a RuntimeError if the process returns an
/// error, otherwise returns the output and the duration
fn run_test_timed(command: &str, test: Test) -> Result<TestResult, RunnerError> {
    let mut command = Command::new(command)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let now = Instant::now();

    if let Some(stdin) = command.stdin.as_mut().take() {
        stdin.write_all(test.input.as_bytes()).unwrap();
    }

    let output = command.wait_with_output().unwrap();

    if output.status.success() {
        Ok(test.make_result(String::from_utf8(output.stdout).unwrap(), now.elapsed()))
    } else {
        Err(RunnerError::RuntimeError(
            String::from_utf8(output.stdout).unwrap(),
        ))
    }
}
