use acm::models::{
    forms::{GenerateTestsForm, RunnerCustomProblemInputForm, RunnerForm},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use actix_web::rt::task;
use async_trait::async_trait;
use std::{collections::BTreeSet};

use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::{Config, Engine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

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

struct MyState {
    limits: StoreLimits,
    wasi: WasiCtx,
}

/// Runs a command with a specified input, returning a RuntimeError if the process returns an
/// error, otherwise returns the output and the duration
async fn run_test_timed(command: &str, test: Test) -> Result<TestResult, RunnerError> {
    let max_runtime = test.max_runtime;

    match run_command(command, &test.input, max_runtime).await {
        Ok((output, fuel)) => Ok(test.make_result(output, fuel)),
        Err(RunnerError::RuntimeError { message }) => {
            Ok(test.make_result_error(message, max_runtime.unwrap_or(MAX_FUEL)))
        }
        Err(e) => Err(e),
    }
}

const MAX_MEMORY: usize = 1 << 26; // 64MB
const MAX_FUEL: i64 = 1 << 32;

async fn run_command(
    command: &str,
    input: &str,
    fuel: Option<i64>,
) -> Result<(String, u64), RunnerError> {
    let command = command.to_string();
    let input = input.to_string();
    task::spawn_blocking(move || {
        let mut config = Config::default();
        config.consume_fuel(true);
        config
            .cache_config_load("./wasmtime-cache.toml")
            .expect("Failed to load cache configuration");

        let engine = Engine::new(&config).unwrap();

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi).map_err(
            |_| RunnerError::InternalServerError {
                message: "Failed to add wasi runtime to linker".to_string(),
            },
        )?;

        let stdin = ReadPipe::from(input);
        let stdout = WritePipe::new_in_memory();

        let mut store = Store::new(
            &engine,
            MyState {
                wasi: WasiCtxBuilder::new()
                    .stdin(Box::new(stdin.clone()))
                    .stdout(Box::new(stdout.clone()))
                    .build(),
                limits: StoreLimitsBuilder::new()
                    .memory_size(MAX_MEMORY)
                    .instances(2)
                    .build(),
            },
        );

        store
            .add_fuel(fuel.unwrap_or(MAX_FUEL) as u64)
            .expect("Failed to add fuel");
        store.limiter(|state| &mut state.limits);

        // Instantiate our module with the imports we've created, and run it.
        let module =
            Module::from_file(&engine, command).map_err(|_| RunnerError::InternalServerError {
                message: "Failed to open file".to_string(),
            })?;

        linker
            .module(&mut store, "", &module)
            .map_err(|_| RunnerError::InternalServerError {
                message: "Failed to link file".to_string(),
            })?;

        let res = linker
            .get_default(&mut store, "")
            .unwrap()
            .typed::<(), (), _>(&store)
            .unwrap()
            .call(&mut store, ());

        let fuel = store.fuel_consumed().unwrap_or(0);

        if let Err(trap) = res {
            return Err(RunnerError::RuntimeError {
                message: trap.display_reason().to_string(),
            });
        }

        // so we can read from the writepipe
        drop(store);
        let bytes = stdout.try_into_inner().unwrap().into_inner();
        let message = String::from_utf8_lossy(&bytes).to_string();

        Ok((message, fuel))
    })
    .await
    .map_err(|_| RunnerError::InternalServerError {
        message: "Failed to create thread".to_string(),
    })?
}
