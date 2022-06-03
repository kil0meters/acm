use acm::models::{
    forms::{GenerateTestsForm, RunnerCustomProblemInputForm, RunnerForm},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use async_trait::async_trait;
use std::process::Stdio;
use std::time::Instant;
use tokio::{io::AsyncWriteExt, process::Command};

mod cplusplus;

pub use cplusplus::CPlusPlus;

#[async_trait]
pub trait Runner {
    async fn run_tests(&self, form: RunnerForm) -> Result<RunnerResponse, RunnerError>;
    async fn generate_tests(&self, form: GenerateTestsForm) -> Result<Vec<Test>, RunnerError>;
    async fn run_custom_input(
        &self,
        form: RunnerCustomProblemInputForm,
    ) -> Result<TestResult, RunnerError>;
}

/// Runs a command with a specified input, returning a RuntimeError if the process returns an
/// error, otherwise returns the output and the duration
async fn run_test_timed(command: &str, test: Test) -> Result<TestResult, RunnerError> {
    let now = Instant::now();
    let output = run_command(command, &test.input).await?;
    Ok(test.make_result(output, now.elapsed()))
}

async fn run_command(command: &str, input: &str) -> Result<String, RunnerError> {
    let mut command = Command::new("wasmer")
        .args(&["run", command])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = command.stdin.as_mut().take() {
        stdin.write_all(input.as_bytes()).await?;
    }

    let output = command.wait_with_output().await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(RunnerError::RuntimeError(
            String::from_utf8_lossy(&output.stdout).to_string(),
        ))
    }
}
