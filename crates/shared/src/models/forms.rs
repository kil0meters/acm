use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use wasm_memory::WasmFunctionCall;

use crate::models::test::Test;

#[derive(Deserialize, Serialize)]
pub struct SubmitJob {
    pub problem_id: i64,
    pub user_id: i64,
    pub implementation: String,
    pub runtime_multiplier: Option<f64>,
    pub tests: Vec<Test>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GenerateTestsJob {
    pub reference: String,
    pub user_id: i64,
    pub inputs: Vec<WasmFunctionCall>,
}

// TODO: Make naming less bad
#[derive(Clone, Deserialize, Serialize)]
pub struct CustomInputJob {
    pub problem_id: i64,
    pub user_id: i64,
    pub reference: String,
    pub implementation: String,
    pub runtime_multiplier: Option<f64>,
    pub input: WasmFunctionCall,
}

#[derive(Deserialize, Serialize)]
pub struct FirstTimeCompletionsForm {
    pub since: Option<NaiveDateTime>,
}
