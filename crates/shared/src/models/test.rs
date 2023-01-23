use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use wasm_memory::{FunctionValue, WasmFunctionCall};

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
pub struct Test {
    #[serde(default)]
    pub id: i64,
    pub index: i64,
    pub max_fuel: Option<i64>,
    pub input: WasmFunctionCall,
    pub expected_output: FunctionValue,
}

impl FromRow<'_, SqliteRow> for Test {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let input: String = row.try_get("input")?;
        let expected_output: String = row.try_get("expected_output")?;

        Ok(Test {
            id: row.try_get("id")?,
            index: row.try_get("test_number")?,
            max_fuel: row.try_get("max_runtime")?,
            input: serde_json::from_str(&input).map_err(|e| sqlx::Error::ColumnDecode {
                index: "input".into(),
                source: Box::new(e),
            })?,
            expected_output: serde_json::from_str(&expected_output).map_err(|e| {
                sqlx::Error::ColumnDecode {
                    index: "expected_output".into(),
                    source: Box::new(e),
                }
            })?,
        })
    }
}

impl Test {
    pub fn make_result(self, output: FunctionValue, fuel: u64) -> TestResult {
        TestResult {
            id: self.id,
            index: self.index,
            success: output == self.expected_output,
            input: self.input,
            expected_output: self.expected_output,
            output: Some(output),
            error: None,
            max_fuel: self.max_fuel,
            fuel: fuel as i64,
        }
    }

    pub fn make_result_error(self, error: String, fuel: u64) -> TestResult {
        TestResult {
            id: self.id,
            index: self.index,
            success: false,
            input: self.input,
            expected_output: self.expected_output,
            output: None,
            error: Some(error),
            max_fuel: self.max_fuel,
            fuel: fuel as i64,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq)]
pub struct TestResult {
    #[serde(default)]
    pub id: i64,
    pub index: i64,
    pub success: bool,
    pub input: WasmFunctionCall,
    pub expected_output: FunctionValue,
    pub output: Option<FunctionValue>,
    pub fuel: i64,
    pub error: Option<String>,
    pub max_fuel: Option<i64>,
}

impl Eq for TestResult {}

impl PartialOrd for TestResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for TestResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl FromRow<'_, SqliteRow> for TestResult {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let input: String = row.try_get("input")?;
        let expected_output: String = row.try_get("expected_output")?;
        let output: String = row.try_get("output")?;

        Ok(TestResult {
            id: row.try_get("id")?,
            index: row.try_get("test_number")?,
            success: row.try_get("success")?,
            input: serde_json::from_str(&input).map_err(|e| sqlx::Error::ColumnDecode {
                index: "input".into(),
                source: Box::new(e),
            })?,
            expected_output: serde_json::from_str(&expected_output).map_err(|e| {
                sqlx::Error::ColumnDecode {
                    index: "expected_output".into(),
                    source: Box::new(e),
                }
            })?,
            output: serde_json::from_str(&output).map_err(|e| sqlx::Error::ColumnDecode {
                index: "output".into(),
                source: Box::new(e),
            })?,
            fuel: row.try_get("runtime")?,
            error: row.try_get("error")?,
            max_fuel: row.try_get("max_runtime")?,
        })
    }
}
