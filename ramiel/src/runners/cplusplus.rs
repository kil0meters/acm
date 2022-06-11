use acm::models::{
    forms::{GenerateTestsForm, RunnerCustomProblemInputForm, RunnerForm},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use futures::future::join_all;
use async_trait::async_trait;
use std::{path::Path, time::Instant};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

use super::{run_command, run_test_timed, Runner, TestResults};

pub struct CPlusPlus;

#[async_trait]
impl Runner for CPlusPlus {
    async fn run_tests(&self, form: RunnerForm) -> Result<RunnerResponse, RunnerError> {
        let prefix = format!("/tmp/acm/submissions/{}/{}", form.username, form.problem_id);

        let command = compile_problem(&prefix, &form.implementation, &form.runner).await?;

        let mut test_results = TestResults::new();

        let start = Instant::now();

        let tests = join_all(form.tests.into_iter().map(|test| async {
            run_test_timed(&command, test).await
        })).await;

        for test in tests {
            test_results.insert(test?);
        }

        test_results.runtime = start.elapsed().as_millis().try_into().unwrap();

        Ok(test_results.into())
    }

    async fn generate_tests(&self, form: GenerateTestsForm) -> Result<Vec<Test>, RunnerError> {
        let prefix = format!("/tmp/acm/problem_editor/{}", form.username);

        let command = compile_problem(&prefix, &form.reference, &form.runner).await?;

        let mut outputs = Vec::new();
        let mut i = 0;
        for input in form.inputs.into_iter() {
            let output = run_command(&command, &input).await?;
            outputs.push(Test {
                id: 0,
                index: i,
                input,
                expected_output: output,
            });

            i += 1;
        }

        Ok(outputs)
    }

    async fn run_custom_input(
        &self,
        form: RunnerCustomProblemInputForm,
    ) -> Result<TestResult, RunnerError> {
        let reference_prefix = format!(
            "/tmp/acm/custom_input/{}/{}/reference/",
            form.username, form.problem_id
        );
        let implementation_prefix = format!(
            "/tmp/acm/custom_input/{}/{}/implementation/",
            form.username, form.problem_id
        );

        let reference_command =
            compile_problem(&reference_prefix, &form.reference, &form.runner).await?;
        let implementation_command =
            compile_problem(&implementation_prefix, &form.implementation, &form.runner).await?;

        let expected_output = run_command(&reference_command, &form.input).await?;

        let test = Test {
            id: 0,
            index: 0,
            input: form.input,
            expected_output,
        };

        run_test_timed(&implementation_command, test).await
    }
}

async fn compile_problem(
    prefix: &str,
    implementation: &str,
    runner: &str,
) -> Result<String, RunnerError> {
    let runner_filename = format!("{prefix}/runner.cpp");
    let wasm_filename = format!("{prefix}/out.wasm");
    let executable_filename = format!("{prefix}/out.wasmu");
    let implementation_filename = format!("{prefix}/implementation.cpp");

    // If the previous submission was successful and unchanged, go ahead without compiling.
    if Path::new(&executable_filename).exists() {
        if let Some(mut file) = File::open(&implementation_filename).await.ok() {
            let mut data: Vec<u8> = vec![];
            file.read_buf(&mut data).await?;

            // Using md5 here is fine since everything is scoped to the username of the account. The
            // worst anyone could do is trick the server into not compiling their code. Main benefit of
            // using it over something like sha256 is speed and hash length.
            let old_hash = md5::compute(data);
            let new_hash = md5::compute(implementation.as_bytes());

            if old_hash == new_hash {
                return Ok(executable_filename);
            }
        }
    }

    fs::create_dir_all(prefix).await?;

    File::create(&runner_filename)
        .await?
        .write(runner.as_bytes())
        .await?;

    File::create(&implementation_filename)
        .await?
        .write(implementation.as_bytes())
        .await?;

    let output = Command::new("/opt/wasi-sdk/bin/clang++")
        .args([
            "-O2",
            "-Wall",
            "-Wextra",
            "-Wpedantic",
            &runner_filename,
            &implementation_filename,
            "-o",
            &wasm_filename,
        ])
        .output()
        .await?;

    if !output.status.success() {
        match fs::remove_file(&executable_filename).await {
            Ok(_) => {}
            Err(_) => {}
        };

        return Err(parse_cplusplus_error(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let output = Command::new("wasmer")
        .args([
            "compile",
            "--cranelift",
            &wasm_filename,
            "-o",
            &executable_filename,
        ])
        .output()
        .await?;

    if output.status.success() {
        Ok(executable_filename)
    } else {
        Err(RunnerError::InternalServerError(
            "Failed to compile.".to_string(),
        ))
    }
}

fn parse_cplusplus_error(err: String) -> RunnerError {
    RunnerError::CompilationError {
        line: 10,
        message: err,
    }
}
