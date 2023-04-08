use async_trait::async_trait;
use futures::future::join_all;
use shared::models::{
    forms::{CustomInputJob, GenerateTestsJob, SubmitJob},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use std::{collections::HashSet, path::Path};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

use super::{run_command, run_test_timed, Runner, TestResults};

pub struct CPlusPlus;

#[async_trait]
impl Runner for CPlusPlus {
    async fn run_tests(&self, form: SubmitJob) -> Result<RunnerResponse, RunnerError> {
        let prefix = format!("/tmp/acm/submissions/{}/{}", form.user_id, form.problem_id);

        let function_names = form
            .tests
            .iter()
            .map(|test| test.input.name.as_str())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let implementation = process_file(&form.implementation, &function_names);

        let command = compile_problem(&prefix, &implementation).await?;

        // MAYBE WHEN WE HAVE MORE RAM
        // let tests = join_all(
        //     form.tests
        //         .into_iter()
        //         .map(|test| async { run_test_timed(&command, test).await }),
        // )
        // .await;

        // SAD SOLUTION FOR NOW
        let mut tests = vec![];
        for mut test in form.tests {
            test.adjust_runtime(form.runtime_multiplier);
            tests.push(run_test_timed(&command, test).await);
        }

        let mut total_runtime = 0;

        let mut test_results = TestResults::new();

        for test in tests {
            let test = test?;
            total_runtime += test.fuel;
            test_results.insert(test);
        }

        test_results.runtime = total_runtime;

        Ok(test_results.into())
    }

    async fn generate_tests(&self, form: GenerateTestsJob) -> Result<Vec<Test>, RunnerError> {
        let prefix = format!("/tmp/acm/problem_editor/{}", form.user_id);

        // TODO actually get unique function names from tests
        let reference = process_file(&form.reference, &[&form.inputs[0].name]);
        let command = compile_problem(&prefix, &reference).await?;

        let mut outputs = Vec::new();
        let mut i = 0;
        for input in form.inputs.into_iter() {
            let (output, fuel) = run_command(&command, input.clone(), None).await?;
            outputs.push(Test {
                id: 0,
                index: i,
                max_fuel: Some(fuel as i64),
                input,
                expected_output: output,
            });

            i += 1;
        }

        Ok(outputs)
    }

    async fn run_custom_input(&self, form: CustomInputJob) -> Result<TestResult, RunnerError> {
        let reference_prefix = format!(
            "/tmp/acm/custom_input/{}/{}/reference",
            form.user_id, form.problem_id
        );
        let implementation_prefix = format!(
            "/tmp/acm/custom_input/{}/{}/implementation",
            form.user_id, form.problem_id
        );

        let reference = process_file(&form.reference, &[&form.input.name]);
        let implementation = process_file(&form.implementation, &[&form.input.name]);

        // println!("REFERENCE: {reference}");
        // println!("IMPLEMENTATION: {implementation}");

        let reference_command = compile_problem(&reference_prefix, &reference).await?;
        let implementation_command =
            compile_problem(&implementation_prefix, &implementation).await?;

        let (expected_output, fuel) =
            run_command(&reference_command, form.input.clone(), None).await?;

        let mut test = Test {
            id: 0,
            index: 0,
            input: form.input,
            expected_output,
            max_fuel: Some(fuel as i64),
        };

        test.adjust_runtime(form.runtime_multiplier);

        run_test_timed(&implementation_command, test).await
    }
}

fn process_file(file: &str, entry_names: &[&str]) -> String {
    let bits_cpp = include_str!("default_header.h");

    let mut new_file = String::new();

    // include headers automatically
    new_file.push_str(bits_cpp);

    let mut attributes_added = String::from(file);

    for entry_name in entry_names {
        attributes_added = add_attribute(&attributes_added, entry_name);
    }

    new_file.push_str(&attributes_added);

    new_file
}

// this is pretty inefficient
fn add_attribute(file: &str, entry_name: &str) -> String {
    let mut new_file = String::new();

    let mut found = false;

    // this is far from perfect obviously but it should be more than sufficient for 99% of cases.
    // we break at the start of the first line that has the entrypoint function name
    for line in file.lines() {
        if !found && line.find(entry_name).is_some() {
            new_file.push_str(&format!(
                "__attribute__((export_name(\"{}\")))\n",
                entry_name
            ));

            found = true;
        }

        new_file.push_str(line);
        new_file.push('\n');
    }

    new_file
}

async fn compile_problem(prefix: &str, implementation: &str) -> Result<String, RunnerError> {
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

    File::create(&implementation_filename)
        .await?
        .write_all(implementation.as_bytes())
        .await?;

    let output = Command::new("/opt/wasi-sdk/bin/clang++")
        .args([
            "-O2",
            "-Wl,--no-entry",
            "-mexec-model=reactor",
            "-Wall",
            "-Wextra",
            "-Wpedantic",
            "-fno-exceptions",
            "-std=c++20",
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
