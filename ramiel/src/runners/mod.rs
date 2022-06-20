use acm::models::{
    forms::{GenerateTestsForm, RunnerCustomProblemInputForm, RunnerForm},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use async_trait::async_trait;
use std::{collections::BTreeSet, process::Stdio};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
    time::{timeout, Duration, Instant},
};

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

struct TestResults {
    failed_tests: BTreeSet<TestResult>,
    passed_tests: BTreeSet<TestResult>,

    runtime: i64,
}

impl TestResults {
    fn new() -> Self {
        Self {
            failed_tests: BTreeSet::new(),
            passed_tests: BTreeSet::new(),
            runtime: 0,
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

impl Into<RunnerResponse> for TestResults {
    fn into(self) -> RunnerResponse {
        let mut tests = Vec::with_capacity(self.failed_tests.len() + self.passed_tests.len());
        let passed = self.failed_tests.is_empty();
        tests.extend(self.failed_tests);
        tests.extend(self.passed_tests);

        RunnerResponse {
            tests,
            runtime: self.runtime,
            passed,
        }
    }
}

/// Runs a command with a specified input, returning a RuntimeError if the process returns an
/// error, otherwise returns the output and the duration
async fn run_test_timed(command: &str, test: Test) -> Result<TestResult, RunnerError> {
    let now = Instant::now();
    let output = run_command(command, &test.input).await?;
    Ok(test.make_result(output, now.elapsed()))
}

async fn run_command(command: &str, input: &str) -> Result<String, RunnerError> {
    let mut child = Command::new("wasmer")
        .args(&["run", command])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut().take() {
        stdin.write_all(input.as_bytes()).await?;
    }

    let exit_status = match timeout(Duration::from_secs(5), child.wait()).await {
        Ok(exit_status) => exit_status?,
        Err(_) => {
            child.kill().await?;
            return Err(RunnerError::TimeoutError {
                message: "Process took too long to execute.".to_string(),
            });
        }
    };

    if exit_status.success() {
        let stdout = child
            .stdout
            .take()
            .expect("child process did not have handle to stdout");

        let mut reader = BufReader::new(stdout);

        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;

        Ok(String::from_utf8_lossy(&bytes).to_string())
    } else {
        let stderr = child
            .stderr
            .take()
            .expect("child process did not have handle to stdout");

        let mut reader = BufReader::new(stderr);

        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;

        Err(RunnerError::RuntimeError {
            message: String::from_utf8_lossy(&bytes).to_string(),
        })
    }
}
