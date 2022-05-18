use super::{run_test_timed, Runner, RunnerError, RunnerResponse};
use acm::models::test::Test;
use async_trait::async_trait;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub struct GPlusPlus {}

impl GPlusPlus {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Runner for GPlusPlus {
    async fn run_tests(
        &self,
        problem_id: i64,
        runner_code: &str,
        implementation_code: &str,
        tests: Vec<Test>,
    ) -> Result<RunnerResponse, RunnerError> {
        let runner_filename = &format!("/tmp/{problem_id}/runner.cpp");
        let implementation_filename = &format!("/tmp/{problem_id}/implementation.cpp");
        let executable_filename = &format!("/tmp/{problem_id}/{problem_id}");

        fs::create_dir_all(&format!("/tmp/{problem_id}")).unwrap();

        File::create(runner_filename)
            .unwrap()
            .write(runner_code.as_bytes())
            .unwrap();
        File::create(implementation_filename)
            .unwrap()
            .write(implementation_code.as_bytes())
            .unwrap();

        let output = Command::new("g++")
            .args([
                "-Wall",
                "-Wextra",
                "-Wpedantic",
                runner_filename,
                implementation_filename,
                "-o",
                executable_filename,
            ])
            .current_dir("/tmp")
            .output()
            .unwrap();

        if output.status.success() {
            run_tests(executable_filename, tests)
        } else {
            Err(parse_gplusplus_error(
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
    }
}

fn run_tests(executable_path: &str, tests: Vec<Test>) -> Result<RunnerResponse, RunnerError> {
    let mut res = RunnerResponse::new();

    for test in tests {
        let test = run_test_timed(executable_path, test)?;
        res.insert(test);
    }

    Ok(res)
}

fn parse_gplusplus_error(err: String) -> RunnerError {
    RunnerError::CompilationError {
        line: 10,
        message: err,
    }
}
