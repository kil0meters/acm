use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::WasmFunctionCall;

trait RuntimeFactor {
    fn get_factor(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AsymptoticComplexity {
    Exponential,
    Quadratic,
    LogLinear,
    Linear,
    Sqrt,
    Log,
    Constant,
}

fn transpose<T>(list: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = list[0].len();
    let mut iters: Vec<_> = list.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

fn normalize(list: &mut Vec<f32>) {
    let min = *list
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = *list
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // simple case
    if min == max {
        for item in list.iter_mut() {
            *item = 1.0;
        }
    } else {
        for item in list.iter_mut() {
            *item = (*item - min) / (max - min);
        }
    }
}

fn calculate_mean_difference(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| (ai - bi).abs())
        .sum::<f32>()
        / a.len() as f32
}

fn normalize_with_sample_function<F>(inputs: &Vec<Vec<(f32, usize)>>, function: F) -> Vec<Vec<f32>>
where
    F: Fn(f32, usize) -> f32,
{
    inputs
        .clone()
        .into_iter()
        .map(|argument| {
            let mut argument = argument
                .into_iter()
                .map(|(factor, i)| factor / function(factor, i))
                .collect();

            normalize(&mut argument);
            argument
        })
        .collect()
}

pub fn estimate_asymptotic_complexity(
    inputs: Vec<WasmFunctionCall>,
    times: Vec<f32>,
) -> Option<AsymptoticComplexity> {
    let inputs = transpose(
        inputs
            .into_iter()
            .enumerate()
            .map(|(i, input)| {
                let mut argument = input
                    .arguments
                    .into_iter()
                    .map(|argument| (argument.scaling_factor(), i))
                    .collect::<Vec<_>>();

                // sort by input difficulty factor
                argument.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

                argument
            })
            .collect::<Vec<_>>(),
    );

    // println!("times: {:?}", times);
    // println!("base data: {:?}\n", inputs);

    let factors = normalize_with_sample_function(&inputs, |_, i| times[i]);
    let exponentials = normalize_with_sample_function(&inputs, |i, _| (i as f32).exp());
    let quadratics = normalize_with_sample_function(&inputs, |i, _| (i as f32 + 1.0).powi(2));
    let log_linears =
        normalize_with_sample_function(&inputs, |i, _| (i as f32 + 1.0) * (i as f32 + 2.0).log2());
    let linears = normalize_with_sample_function(&inputs, |i, _| i as f32 + 1.0);
    let sqrts = normalize_with_sample_function(&inputs, |i, _| (i as f32 + 1.0).sqrt());
    let logs = normalize_with_sample_function(&inputs, |i, _| (i as f32 + 2.0).log2());
    let constants = normalize_with_sample_function(&inputs, |_, _| 1.0);

    // println!("actual: {:?}\n", factors);
    // println!("exponentials: {:?}", exponentials);
    // println!("quadratics: {:?}", quadratics);
    // println!("log_linears: {:?}", log_linears);
    // println!("linears: {:?}", linears);
    // println!("sqrts: {:?}", sqrts);
    // println!("constants: {:?}", constants);

    let mut detected_complexities = vec![];

    for (i, factor) in factors.iter().enumerate() {
        let mut mean_differences = vec![];

        mean_differences.push((
            AsymptoticComplexity::Exponential,
            calculate_mean_difference(&exponentials[i], &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Quadratic,
            calculate_mean_difference(&quadratics[i], &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::LogLinear,
            calculate_mean_difference(&log_linears[i], &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Linear,
            calculate_mean_difference(&linears[i], &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Sqrt,
            calculate_mean_difference(&sqrts[i], &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Log,
            calculate_mean_difference(&logs[i], &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Constant,
            calculate_mean_difference(&constants[i], &factor),
        ));

        // find min index
        let idx = mean_differences
            .iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        detected_complexities.push(*idx);
    }

    // We only give a complexity estimate if we're pretty confident.
    // This saves us from giving some incorrect answers.
    for (complexity, mean_diff) in detected_complexities {
        if mean_diff < 0.1 {
            return Some(complexity);
        }
    }

    None
}
