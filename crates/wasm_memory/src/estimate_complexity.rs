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

fn sort_and_normalize(list: &mut Vec<f32>) {
    list.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    normalize_sorted(list);
}

fn normalize_sorted(list: &mut Vec<f32>) {
    let min = *list.first().unwrap();
    let max = *list.last().unwrap();

    for item in list.iter_mut() {
        *item = (*item - min) / (max - min);
    }
}

fn calculate_mean_difference(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| (ai - bi).abs())
        .sum::<f32>()
        / a.len() as f32
}

pub fn estimate_asymptotic_complexity(
    inputs: Vec<WasmFunctionCall>,
    times: Vec<f32>,
) -> AsymptoticComplexity {
    let inputs = inputs
        .into_iter()
        .zip(times.into_iter())
        .map(|(input, time)| {
            input
                .arguments
                .into_iter()
                .map(|argument| argument.scaling_factor() / time)
                .collect()
        })
        .collect::<Vec<_>>();

    let input_length = inputs.len();

    // transpose, then normalize and sort
    let factors = transpose(inputs)
        .into_iter()
        .map(|mut argument| {
            sort_and_normalize(&mut argument);
            argument
        })
        .collect::<Vec<_>>();

    // This should be possible to precompute somehow.
    let mut exponential = (0..input_length).map(|x| (x as f32).exp()).collect();
    sort_and_normalize(&mut exponential);

    let mut quadratic = (0..input_length).map(|x| (x as f32).powi(2)).collect();
    sort_and_normalize(&mut quadratic);

    let mut log_linear = (0..input_length)
        .map(|x| x as f32 * (x as f32 + 1.0).log2())
        .collect();
    sort_and_normalize(&mut log_linear);

    let mut linear = (0..input_length).map(|x| x as f32).collect();
    sort_and_normalize(&mut linear);

    let mut sqrt = (0..input_length).map(|x| (x as f32).sqrt()).collect();
    sort_and_normalize(&mut sqrt);

    let mut log = (0..input_length).map(|x| (x as f32 + 1.0).log2()).collect();
    sort_and_normalize(&mut log);

    let constant = vec![0.0; input_length];

    let mut detected_complexities = vec![];

    for factor in factors {
        let mut mean_differences = vec![];

        mean_differences.push((
            AsymptoticComplexity::Exponential,
            calculate_mean_difference(&exponential, &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Quadratic,
            calculate_mean_difference(&quadratic, &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::LogLinear,
            calculate_mean_difference(&log_linear, &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Linear,
            calculate_mean_difference(&linear, &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Sqrt,
            calculate_mean_difference(&sqrt, &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Log,
            calculate_mean_difference(&log, &factor),
        ));
        mean_differences.push((
            AsymptoticComplexity::Constant,
            calculate_mean_difference(&constant, &factor),
        ));

        // find min index
        let (detected, _) = mean_differences
            .iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        detected_complexities.push(*detected);
    }

    *detected_complexities.iter().max().unwrap()
}
