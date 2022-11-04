use acm::models::{
    forms::{GenerateTestsForm, RunnerCustomProblemInputForm, RunnerForm},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use async_trait::async_trait;
use futures::future::join_all;
use std::path::Path;
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
        let prefix = format!("/tmp/acm/submissions/{}/{}", form.user_id, form.problem_id);

        let command = compile_problem(&prefix, &form.implementation, &form.runner).await?;

        let mut test_results = TestResults::new();

        let tests = join_all(
            form.tests
                .into_iter()
                .map(|test| async { run_test_timed(&command, test).await }),
        )
        .await;

        let mut total_runtime = 0;

        for test in tests {
            let test = test?;
            total_runtime += test.runtime;
            test_results.insert(test);
        }

        test_results.runtime = total_runtime;

        Ok(test_results.into())
    }

    async fn generate_tests(&self, form: GenerateTestsForm) -> Result<Vec<Test>, RunnerError> {
        let prefix = format!("/tmp/acm/problem_editor/{}", form.user_id);

        let command = compile_problem(&prefix, &form.reference, &form.runner).await?;

        let mut outputs = Vec::new();
        let mut i = 0;
        for input in form.inputs.into_iter() {
            let (output, fuel) = run_command(&command, &input, None).await?;
            outputs.push(Test {
                id: 0,
                index: i,
                max_runtime: Some(((fuel as f64) * form.runtime_multiplier) as i64),
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
            "/tmp/acm/custom_input/{}/{}/reference",
            form.user_id, form.problem_id
        );
        let implementation_prefix = format!(
            "/tmp/acm/custom_input/{}/{}/implementation",
            form.user_id, form.problem_id
        );

        let reference_command =
            compile_problem(&reference_prefix, &form.reference, &form.runner).await?;
        let implementation_command =
            compile_problem(&implementation_prefix, &form.implementation, &form.runner).await?;

        let (expected_output, fuel) = run_command(&reference_command, &form.input, None).await?;

        let test = Test {
            id: 0,
            index: 0,
            input: form.input,
            expected_output,
            max_runtime: Some(((fuel as f64) * form.runtime_multiplier) as i64),
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
    let implementation_filename = format!("{prefix}/implementation.cpp");

    // If the previous submission was successful and unchanged, go ahead without compiling.
    if Path::new(&wasm_filename).exists() {
        if let Some(mut file) = File::open(&implementation_filename).await.ok() {
            let mut data: Vec<u8> = vec![];
            file.read_buf(&mut data).await?;

            // Using md5 here is fine since everything is scoped to the username of the account. The
            // worst anyone could do is trick the server into not compiling their code. Main benefit of
            // using it over something like sha256 is speed and hash length.
            let old_hash = md5::compute(data);
            let new_hash = md5::compute(implementation.as_bytes());

            if old_hash == new_hash {
                return Ok(wasm_filename);
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
            "-fno-exceptions",
            &runner_filename,
            &implementation_filename,
            "-o",
            &wasm_filename,
        ])
        .output()
        .await?;

    if !output.status.success() {
        fs::remove_file(&wasm_filename).await.ok();

        return Err(parse_cplusplus_error(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(wasm_filename)
}

fn parse_cplusplus_error(err: String) -> RunnerError {
    RunnerError::CompilationError {
        line: 10,
        message: err,
    }
}
