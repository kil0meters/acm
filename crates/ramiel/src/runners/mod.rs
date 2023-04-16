use actix_web::rt::task;
use async_trait::async_trait;
use shared::models::{
    forms::{CustomInputJob, GenerateTestsJob, SubmitJob},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
};
use std::collections::BTreeSet;
use wasm_memory::{FunctionValue, WasmFunctionCall};

use wasmtime::{Config, Engine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

mod cplusplus;

pub use cplusplus::CPlusPlus;

#[async_trait]
pub trait Runner {
    async fn run_tests(&self, form: SubmitJob) -> Result<RunnerResponse, RunnerError>;
    async fn generate_tests(&self, form: GenerateTestsJob) -> Result<Vec<Test>, RunnerError>;
    async fn run_custom_input(&self, form: CustomInputJob) -> Result<TestResult, RunnerError>;
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
        if test.success {
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
    // we allow a nice buffer of 20x so the user can more easily debug their program
    let max_runtime = test.max_fuel.map(|x| x * 20);

    match run_command(command, test.input.clone(), max_runtime).await {
        Ok((output, fuel)) => {
            let mut test_result = test.make_result(output, fuel);

            if fuel > test_result.max_fuel.unwrap_or(MAX_FUEL) as u64 {
                test_result.success = false;
                test_result.error = Some("Fuel limit exceeded".to_string())
            }

            Ok(test_result)
        }
        Err(RunnerError::RuntimeError { message }) => {
            Ok(test.make_result_error(message, max_runtime.unwrap_or(MAX_FUEL) as u64))
        }
        Err(e) => Err(e),
    }
}

const MAX_MEMORY: usize = 1 << 29; // 512MB
const MAX_FUEL: i64 = 1 << 48;

async fn run_command(
    command: &str,
    input: WasmFunctionCall,
    fuel: Option<i64>,
) -> Result<(FunctionValue, u64), RunnerError> {
    let command = command.to_string();
    task::spawn_blocking(move || {
        let mut config = Config::default();
        config.consume_fuel(true);
        config
            .cache_config_load("./wasmtime-cache.toml")
            .expect("Failed to load cache configuration");

        let engine = Engine::new(&config).expect("Failed to create engine");

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi).map_err(
            |_| RunnerError::InternalServerError {
                message: "Failed to add wasi runtime to linker".to_string(),
            },
        )?;

        let mut store = Store::new(
            &engine,
            MyState {
                wasi: WasiCtxBuilder::new().build(),
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

        let instance = linker.instantiate(&mut store, &module).map_err(|e| {
            println!("error: {e:?}");
            RunnerError::InternalServerError {
                message: "Failed to instantiate module".to_string(),
            }
        })?;

        let result = input.call(&mut store, &instance);

        match result {
            Ok((res, fuel)) => Ok((res, fuel)),
            Err(e) => Err(RunnerError::RuntimeError {
                message: e.root_cause().to_string(),
            }),
        }
    })
    .await
    .map_err(|e| {
        println!("caught error: {e}");
        return RunnerError::InternalServerError {
            message: "Failed to create thread".to_string(),
        };
    })?
}
