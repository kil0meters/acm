use async_trait::async_trait;
use shared::models::{
    forms::{CustomInputJob, GenerateTestsJob, SubmitJob},
    runner::{CustomInputResponse, Diagnostic, DiagnosticType, RunnerError, RunnerResponse},
    test::Test,
};
use std::{collections::HashSet, iter::Peekable, path::Path, str::Chars};
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

        let implementation = process_file(&form.implementation);

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
            let (test, _) = run_test_timed(&command, test, 50).await?;
            tests.push(test);
        }

        let mut total_runtime = 0;

        let mut test_results = TestResults::new();

        for test in tests {
            total_runtime += test.fuel;
            test_results.insert(test);
        }

        test_results.runtime = total_runtime;

        Ok(test_results.into())
    }

    async fn generate_tests(&self, form: GenerateTestsJob) -> Result<Vec<Test>, RunnerError> {
        let prefix = format!("/tmp/acm/problem_editor/{}", form.user_id);

        // TODO actually get unique function names from tests
        let reference = process_file(&form.reference);
        let command = compile_problem(&prefix, &reference).await?;

        let mut outputs = Vec::new();
        let mut i = 0;
        for input in form.inputs.into_iter() {
            let (output, _, fuel) = run_command(&command, input.clone(), None).await?;
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

    async fn run_custom_input(
        &self,
        form: CustomInputJob,
    ) -> Result<CustomInputResponse, RunnerError> {
        let reference_prefix = format!(
            "/tmp/acm/custom_input/{}/{}/reference",
            form.user_id, form.problem_id
        );
        let implementation_prefix = format!(
            "/tmp/acm/custom_input/{}/{}/implementation",
            form.user_id, form.problem_id
        );

        let reference = process_file(&form.reference);
        let implementation = process_file(&form.implementation);

        // println!("REFERENCE: {reference}");
        // println!("IMPLEMENTATION: {implementation}");

        let reference_command = compile_problem(&reference_prefix, &reference).await?;
        let implementation_command =
            compile_problem(&implementation_prefix, &implementation).await?;

        let (expected_output, _, fuel) =
            run_command(&reference_command, form.input.clone(), None).await?;

        let mut test = Test {
            id: 0,
            index: 0,
            input: form.input,
            expected_output,
            max_fuel: Some(fuel as i64),
        };

        test.adjust_runtime(form.runtime_multiplier);

        // we add a lot of padding so they can potentially print a lot
        let (test_result, stdout) = run_test_timed(&implementation_command, test, 500).await?;

        Ok(CustomInputResponse {
            result: test_result,
            output: stdout,
        })
    }
}

fn process_file(file: &str) -> String {
    let bits_cpp = include_str!("default_header.h");

    let mut new_file = String::new();

    // include headers automatically
    new_file.push_str(bits_cpp);
    new_file.push_str(&file);

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
            "-O3",
            "-Wl,--no-entry",
            "-Wl,--demangle",
            "-Wl,--export-all",
            "-mexec-model=reactor",
            "-msimd128",
            "-Wall",
            "-Wextra",
            "-Wpedantic",
            "-fno-caret-diagnostics",
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

fn parse_number(iter: &mut Peekable<Chars>) -> usize {
    let mut num = 0;

    while let Some(c) = iter.next() {
        if let Some(d) = c.to_digit(10) {
            num = num * 10 + d as usize;
        } else {
            break;
        }
    }

    num
}

/// Returns `None` if the diagnostic is not in the "implementation.cpp" file
///
/// Example format (except we don't actually do the brackets thus far):
/// /tmp/acm/submissions/1/41/implementation.cpp:50:12:{50:16-50:17}: error: no viable conversion from 'int' to 'std::string' (aka 'basic_string<char, char_traits<char>, allocator<char>>')
fn diagnostic_from_str(s: &str) -> Result<Option<Diagnostic>, RunnerError> {
    if s.find(".cpp").is_none() || !s.starts_with("/") {
        return Ok(None);
    }

    let mut iter = s.chars().peekable();

    // go until we find the first colon
    while let Some(c) = iter.next() {
        if c == ':' {
            break;
        }
    }

    // this number comes from the length of the "default_header.h" file
    let mut line = parse_number(&mut iter);
    if line < 39 {
        return Ok(None);
    }

    line -= 39;

    let col = parse_number(&mut iter);

    iter.next();

    let mut error_type = String::new();

    while let Some(c) = iter.next() {
        if c == ':' {
            break;
        }

        error_type.push(c);
    }

    iter.next();

    let diagnostic_type = match error_type.as_str() {
        "error" => DiagnosticType::Error,
        "warning" => DiagnosticType::Warning,
        _ => DiagnosticType::Note,
    };

    let message = iter.collect();

    Ok(Some(Diagnostic {
        line,
        col,
        message,
        diagnostic_type,
    }))
}

fn parse_cplusplus_error(err: String) -> RunnerError {
    let mut diagnostics = vec![];

    println!("{err}");

    for line in err.lines() {
        match diagnostic_from_str(&line) {
            Ok(Some(diagnostic)) => diagnostics.push(diagnostic),
            Ok(None) => {}
            Err(e) => {
                return e;
            }
        }
    }

    RunnerError::CompilationError { diagnostics }
}
