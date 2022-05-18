use acm::models::{
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use async_trait::async_trait;
use std::{
    io::Write,
    process::{Command, Stdio},
    time::Instant,
};

mod gplusplus;

pub use gplusplus::GPlusPlus;

#[async_trait]
pub trait Runner {
    async fn run_tests(
        &self,
        problem_id: i64,
        runner_code: &str,
        test_code: &str,
        tests: Vec<Test>,
    ) -> Result<RunnerResponse, RunnerError>;
}

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
