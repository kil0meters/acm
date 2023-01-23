use anyhow::Result;
use serde::{Deserialize, Serialize};
use shared::models::test::Test;
use sqlx::{FromRow, SqlitePool};
use wasm_memory::{
    ContainerVariant, ContainerVariantType, FunctionType, FunctionValue, WasmFunctionCall,
};

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq, FromRow)]
pub struct OldTest {
    #[serde(default)]
    pub id: i64,
    pub test_number: i64,
    pub max_runtime: Option<i64>,
    pub input: String,
    pub expected_output: String,
}

fn replicate(function_name: &str, test: OldTest) -> Result<Test> {
    let (_line1, grid) = test.input.split_once("\n").unwrap();

    let input_grid = grid
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Char(ContainerVariant::Grid(input_grid))],
        FunctionType::Char(ContainerVariantType::Grid),
    );

    let output_grid = test
        .expected_output
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let result = FunctionValue::Char(ContainerVariant::Grid(output_grid));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn two_strings(function_name: &str, test: OldTest) -> Result<Test> {
    let (s1, s2) = test.input.split_once("\n").unwrap();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::String(ContainerVariant::Single(s1.trim().to_string())),
            FunctionValue::String(ContainerVariant::Single(s2.trim().to_string())),
        ],
        FunctionType::String(ContainerVariantType::Single),
    );

    let result = FunctionValue::String(ContainerVariant::Single(
        test.expected_output.trim().to_string(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn string_int(function_name: &str, test: OldTest) -> Result<Test> {
    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::String(ContainerVariant::Single(
            test.input.trim().to_string(),
        ))],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse()?,
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn string_bool(function_name: &str, test: OldTest) -> Result<Test> {
    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::String(ContainerVariant::Single(
            test.input.trim().to_string(),
        ))],
        FunctionType::Bool(ContainerVariantType::Single),
    );

    let result = FunctionValue::Bool(ContainerVariant::Single(
        if test.expected_output.trim() == "Odd." {
            true
        } else {
            false
        },
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn vec_i32(function_name: &str, test: OldTest) -> Result<Test> {
    let (_first_line, rest) = test.input.split_once("\n").unwrap();

    let nums = rest
        .trim()
        .split_whitespace()
        .map(|each| each.trim().parse().unwrap())
        .collect::<Vec<i32>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Int(ContainerVariant::List(nums))],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn need_for_speed(function_name: &str, test: OldTest) -> Result<Test> {
    let (first, rest) = test.input.split_once("\n").unwrap();

    let (_n, t) = first.split_once(" ").unwrap();

    let t = t.parse::<i32>()?;

    let mut d = vec![];
    let mut s = vec![];

    for line in rest.lines() {
        let (d_i, s_i) = line.split_once(" ").unwrap();

        d.push(d_i.parse()?);
        s.push(s_i.parse()?);
    }

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Int(ContainerVariant::Single(t)),
            FunctionValue::Int(ContainerVariant::List(d)),
            FunctionValue::Int(ContainerVariant::List(s)),
        ],
        FunctionType::Double(ContainerVariantType::Single),
    );

    let result = FunctionValue::Double(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn maximum_nonadjacent_sum(function_name: &str, test: OldTest) -> Result<Test> {
    let (first, rest) = test.input.split_once("\n").unwrap();

    let num_nodes = first.trim().parse::<i32>()?;

    let mut values = vec![];

    for line in rest.lines().take(num_nodes as usize) {
        values.push(line.trim().parse::<i32>()?);
    }

    let mut edges = vec![vec![]; num_nodes as usize];

    for line in rest.trim().lines().skip(num_nodes as usize) {
        let (u, v) = line.split_once(" ").unwrap();

        let u = u.parse::<i32>()?;
        let v = v.parse::<i32>()?;

        edges[u as usize].push(v);
        edges[v as usize].push(u);
    }

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Int(ContainerVariant::Graph(edges)),
            FunctionValue::Int(ContainerVariant::List(values)),
        ],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn tree_vertex_cover(function_name: &str, test: OldTest) -> Result<Test> {
    let (first, rest) = test.input.split_once("\n").unwrap();

    let num_nodes = first.trim().parse::<i32>()?;

    let mut edges = vec![vec![]; num_nodes as usize];

    for line in rest.trim().lines() {
        let (u, v) = line.split_once(" ").unwrap();

        let u = u.parse::<i32>()?;
        let v = v.parse::<i32>()?;

        edges[u as usize - 1].push(v - 1);
        edges[v as usize - 1].push(u - 1);
    }

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Int(ContainerVariant::Graph(edges))],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn rainbow_roads(function_name: &str, test: OldTest) -> Result<Test> {
    let (first, rest) = test.input.split_once("\n").unwrap();

    let num_nodes = first.trim().parse::<i32>()?;

    let mut weights = vec![];
    let mut edges = vec![vec![]; num_nodes as usize];

    for line in rest.trim().lines() {
        let elements = line
            .split(" ")
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<i32>>();

        let u = elements[0] - 1;
        let v = elements[1] - 1;

        edges[u as usize].push(v);
        edges[v as usize].push(u);

        weights.push(elements[2]);
    }

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Int(ContainerVariant::Graph(edges)),
            FunctionValue::Int(ContainerVariant::List(weights)),
        ],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let (_first, rest) = test.expected_output.split_once("\n").unwrap();

    let result = FunctionValue::Int(ContainerVariant::List(
        rest.lines().map(|x| x.parse().unwrap()).collect(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn convert_raw_string(function_name: &str, test: OldTest) -> Result<Test> {
    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::String(ContainerVariant::Single(test.input))],
        FunctionType::String(ContainerVariantType::Single),
    );

    let result = FunctionValue::String(ContainerVariant::Single(test.expected_output));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn int_int(function_name: &str, test: OldTest) -> Result<Test> {
    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Int(ContainerVariant::Single(
            test.input.trim().parse().unwrap(),
        ))],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn int_string(function_name: &str, test: OldTest) -> Result<Test> {
    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Int(ContainerVariant::Single(
            test.input.trim().parse().unwrap(),
        ))],
        FunctionType::String(ContainerVariantType::Single),
    );

    let result = FunctionValue::String(ContainerVariant::Single(test.expected_output));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn coloring_contention(function_name: &str, test: OldTest) -> Result<Test> {
    let (first, rest) = test.input.split_once("\n").unwrap();

    let (n, _m) = first.split_once(" ").unwrap();

    let n = n.trim().parse().unwrap();
    // let num_nodes = num_nodes.trim().parse::<i32>()?;

    let mut edges = vec![vec![]; n as usize];

    for line in rest.trim().lines() {
        let (u, v) = line.split_once(" ").unwrap();

        let u = u.trim().parse::<i32>().unwrap() - 1;
        let v = v.trim().parse::<i32>().unwrap() - 1;

        edges[u as usize].push(v);
        edges[v as usize].push(u);
    }

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Int(ContainerVariant::Single(n)),
            FunctionValue::Int(ContainerVariant::Graph(edges)),
        ],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn delayed_work(function_name: &str, test: OldTest) -> Result<Test> {
    let input = test
        .input
        .split(" ")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<f64>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Double(ContainerVariant::Single(input[0])),
            FunctionValue::Double(ContainerVariant::Single(input[1])),
            FunctionValue::Double(ContainerVariant::Single(input[2])),
        ],
        FunctionType::Double(ContainerVariantType::Single),
    );

    // we need overrides because of precision here
    let result = if test.input.trim() == "314 159 264" {
        FunctionValue::Double(ContainerVariant::Single(7276.142857142857))
    } else if test.input.trim() == "271 828 1" {
        FunctionValue::Double(ContainerVariant::Single(947.3924050632912))
    } else {
        FunctionValue::Double(ContainerVariant::Single(
            test.expected_output.trim().parse()?,
        ))
    };

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn fear_factoring(function_name: &str, test: OldTest) -> Result<Test> {
    let input = test
        .input
        .split(" ")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<i64>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Long(ContainerVariant::Single(input[0])),
            FunctionValue::Long(ContainerVariant::Single(input[1])),
        ],
        FunctionType::Long(ContainerVariantType::Single),
    );

    // we need overrides because of precision here
    let result = FunctionValue::Long(ContainerVariant::Single(
        test.expected_output.trim().parse()?,
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn mission_improbable(function_name: &str, test: OldTest) -> Result<Test> {
    let (_line1, grid) = test.input.split_once("\n").unwrap();

    let input_grid = grid
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Long(ContainerVariant::Grid(input_grid))],
        FunctionType::Long(ContainerVariantType::Single),
    );

    let result = FunctionValue::Long(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn kth_subtree(function_name: &str, test: OldTest) -> Result<Test> {
    let (first, rest) = test.input.split_once("\n").unwrap();

    let (n, k) = first.split_once(" ").unwrap();

    let n = n.trim().parse().unwrap();
    let k = k.trim().parse().unwrap();

    let mut edges = vec![vec![]; n as usize];

    for line in rest.trim().lines() {
        let (u, v) = line.split_once(" ").unwrap();

        let u = u.trim().parse::<i32>().unwrap() - 1;
        let v = v.trim().parse::<i32>().unwrap() - 1;

        edges[u as usize].push(v);
        edges[v as usize].push(u);
    }

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Long(ContainerVariant::Single(k)),
            FunctionValue::Int(ContainerVariant::Single(n)),
            FunctionValue::Int(ContainerVariant::Graph(edges)),
        ],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn papercuts(function_name: &str, test: OldTest) -> Result<Test> {
    let (line1, line2) = test.input.split_once("\n").unwrap();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::String(ContainerVariant::Single(line1.trim().to_string())),
            FunctionValue::String(ContainerVariant::Single(line2.trim().to_string())),
        ],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn rectangles(function_name: &str, test: OldTest) -> Result<Test> {
    let (_line1, grid) = test.input.split_once("\n").unwrap();

    let input_grid = grid
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Int(ContainerVariant::Grid(input_grid))],
        FunctionType::Long(ContainerVariantType::Single),
    );

    let result = FunctionValue::Long(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn longest_common_subsequence(function_name: &str, test: OldTest) -> Result<Test> {
    let (line1, rest) = test.input.split_once("\n").unwrap();

    let (_n, k) = line1.split_once(" ").unwrap();

    let k = k.trim().parse()?;

    let input_grid = rest
        .trim()
        .lines()
        .map(|x| x.trim().to_string())
        .collect::<Vec<_>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Int(ContainerVariant::Single(k)),
            FunctionValue::String(ContainerVariant::List(input_grid)),
        ],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn meeting(function_name: &str, test: OldTest) -> Result<Test> {
    let (line1, rest) = test.input.split_once("\n").unwrap();

    let elements = line1
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let n = elements[0];
    let k = elements[1];
    let t = elements[2];

    let a = rest
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse())
        .collect::<Result<_, _>>()?;

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![
            FunctionValue::Int(ContainerVariant::Single(n)),
            FunctionValue::Int(ContainerVariant::Single(k)),
            FunctionValue::Int(ContainerVariant::Single(t)),
            FunctionValue::Int(ContainerVariant::List(a)),
        ],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn poker_hand(function_name: &str, test: OldTest) -> Result<Test> {
    let elements = test
        .input
        .trim()
        .split_whitespace()
        .map(|x| x.trim().to_string())
        .collect::<Vec<_>>();

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::String(ContainerVariant::List(elements))],
        FunctionType::Int(ContainerVariantType::Single),
    );

    let result = FunctionValue::Int(ContainerVariant::Single(
        test.expected_output.trim().parse().unwrap(),
    ));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

fn star_arrangements(function_name: &str, test: OldTest) -> Result<Test> {
    let input = test.input.trim().parse()?;

    let input_call = WasmFunctionCall::new(
        function_name,
        vec![FunctionValue::Int(ContainerVariant::Single(input))],
        FunctionType::String(ContainerVariantType::Single),
    );

    let result = FunctionValue::String(ContainerVariant::Single(test.expected_output.to_string()));

    Ok(Test {
        id: test.id,
        index: test.test_number,
        max_fuel: test.max_runtime,
        input: input_call,
        expected_output: result,
    })
}

async fn convert_problem_tests(
    problem_id: i64,
    function_name: &str,
    conversion_function: Box<dyn Fn(&str, OldTest) -> Result<Test>>,
) -> Result<()> {
    let pool = SqlitePool::connect("./db.sqlite").await?;

    let tests: Vec<OldTest> = sqlx::query_as(
        "SELECT id, test_number, max_runtime, input, expected_output
        FROM tests
        WHERE problem_id = ?",
    )
    .bind(problem_id)
    .fetch_all(&pool)
    .await?;

    let tests = tests
        .into_iter()
        .map(|test| conversion_function(function_name, test).unwrap())
        .collect::<Vec<_>>();

    for test in tests {
        println!("Converting test {}", test.id);
        sqlx::query(
            "
            UPDATE tests
            SET
            input = ?,
            expected_output = ?
            WHERE id = ?
        ",
        )
        .bind(serde_json::to_string(&test.input)?)
        .bind(serde_json::to_string(&test.expected_output)?)
        .bind(test.id)
        .execute(&pool)
        .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    convert_problem_tests(15, "replicate", Box::new(replicate)).await?; // rename solution to replicate and change to vector<vector<char>> out return type
    convert_problem_tests(14, "computeC", Box::new(need_for_speed)).await?;
    convert_problem_tests(13, "maxNonadjacentSum", Box::new(maximum_nonadjacent_sum)).await?;
    convert_problem_tests(12, "treeVCSize", Box::new(tree_vertex_cover)).await?;
    convert_problem_tests(11, "permuteDigits", Box::new(two_strings)).await?;
    convert_problem_tests(10, "maxNonconsecutiveSum", Box::new(vec_i32)).await?;
    convert_problem_tests(9, "branch", Box::new(convert_raw_string)).await?;
    convert_problem_tests(8, "rainbowRoads", Box::new(rainbow_roads)).await?;
    convert_problem_tests(7, "ways", Box::new(int_int)).await?;
    convert_problem_tests(6, "treeCount", Box::new(convert_raw_string)).await?;
    convert_problem_tests(5, "deadEndDetector", Box::new(convert_raw_string)).await?;
    convert_problem_tests(4, "goodOrBad", Box::new(convert_raw_string)).await?;
    convert_problem_tests(2, "fibonacci", Box::new(int_int)).await?;
    convert_problem_tests(1, "fizzBuzz", Box::new(int_string)).await?;

    convert_problem_tests(17, "delayed_work", Box::new(delayed_work)).await?;
    convert_problem_tests(18, "dominating_duos", Box::new(vec_i32)).await?;
    convert_problem_tests(20, "fear_factor", Box::new(fear_factoring)).await?;
    convert_problem_tests(23, "kth_subtree", Box::new(kth_subtree)).await?;
    convert_problem_tests(26, "mission_imporbable", Box::new(mission_improbable)).await?;
    convert_problem_tests(28, "paper_cuts", Box::new(papercuts)).await?;
    convert_problem_tests(29, "permutation", Box::new(convert_raw_string)).await?;
    convert_problem_tests(31, "rectangles", Box::new(rectangles)).await?;
    convert_problem_tests(34, "straight_shot", Box::new(convert_raw_string)).await?;
    convert_problem_tests(35, "ant_typing", Box::new(string_int)).await?;

    convert_problem_tests(16, "coloring_contention", Box::new(coloring_contention)).await?;
    convert_problem_tests(19, "excellence", Box::new(vec_i32)).await?;
    convert_problem_tests(21, "gravity", Box::new(replicate)).await?; // change to vector<vector<char>> with out parameter
    convert_problem_tests(22, "forbidden_zero", Box::new(int_int)).await?;
    #[rustfmt::skip]
    convert_problem_tests(24, "longest_common_subsequence", Box::new(longest_common_subsequence)).await?;
    convert_problem_tests(25, "meeting", Box::new(meeting)).await?;
    convert_problem_tests(27, "odd_palindrome", Box::new(string_bool)).await?;
    convert_problem_tests(30, "poker_hand", Box::new(poker_hand)).await?; // change function signature to take vector<string>
    convert_problem_tests(32, "runes", Box::new(string_int)).await?;
    convert_problem_tests(33, "star_arrangements", Box::new(star_arrangements)).await?;

    Ok(())
}
