mod builder_arguments_pattern;
mod default_struct_arguments_pattern;
mod explicit_arguments_pattern;
use std::time::Instant;

const RUNS: i64 = 20;
const REPETITIONS_PER_RUN: i64 = 100000000;

fn execute_my_func_benchmark() {
    println!("\nRunning execute_my_func_benchmark...");
    let mut default_durations_micros = Vec::new();
    let mut explicit_durations_micros = Vec::new();
    let mut builder_durations_micros = Vec::new();
    for _ in 0..RUNS {
        {
            let start = Instant::now();
            let result =
                default_struct_arguments_pattern::exercise_my_func_calls(REPETITIONS_PER_RUN);
            let elapsed = start.elapsed();
            default_durations_micros.push(elapsed.as_micros());
            println!(
                "(default arguments run)  elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }
        {
            let start = Instant::now();
            let result = explicit_arguments_pattern::exercise_my_func_calls(REPETITIONS_PER_RUN);
            let elapsed = start.elapsed();
            explicit_durations_micros.push(elapsed.as_micros());
            println!(
                "(explicit arguments run) elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }

        {
            let start = Instant::now();
            let result = builder_arguments_pattern::exercise_my_func_calls(REPETITIONS_PER_RUN);
            let elapsed = start.elapsed();
            builder_durations_micros.push(elapsed.as_micros());
            println!(
                "(builder arguments run)  elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }
    }

    println!("\nAgregated results: (my_func inlined)");
    println!(
        "(default arguments)  mean runtime: {:.3}ms, std_dev: {:.3}ms",
        mean(&default_durations_micros) / 1000.0,
        std_deviation(&default_durations_micros) / 1000.0
    );
    println!(
        "(explicit arguments) mean runtime: {:.3}ms, std_dev: {:.3}ms",
        mean(&explicit_durations_micros) / 1000.0,
        std_deviation(&explicit_durations_micros) / 1000.0
    );
    println!(
        "(builder arguments)  mean runtime: {:.3}ms, std_dev: {:.3}ms",
        mean(&builder_durations_micros) / 1000.0,
        std_deviation(&builder_durations_micros) / 1000.0
    );
}

fn execute_my_func_never_inlined_benchmark() {
    println!("\nRunning execute_my_func_never_inlined_benchmark...");
    let mut default_durations_micros = Vec::new();
    let mut explicit_durations_micros = Vec::new();
    let mut builder_durations_micros = Vec::new();
    for _ in 0..RUNS {
        {
            let start = Instant::now();
            let result = default_struct_arguments_pattern::exercise_my_func_never_inlined_calls(
                REPETITIONS_PER_RUN,
            );
            let elapsed = start.elapsed();
            default_durations_micros.push(elapsed.as_micros());
            println!(
                "(default arguments run)  elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }
        {
            let start = Instant::now();
            let result = explicit_arguments_pattern::exercise_my_func_never_inlined_calls(
                REPETITIONS_PER_RUN,
            );
            let elapsed = start.elapsed();
            explicit_durations_micros.push(elapsed.as_micros());
            println!(
                "(explicit arguments run) elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }

        {
            let start = Instant::now();
            let result = builder_arguments_pattern::exercise_my_func_never_inlined_calls(
                REPETITIONS_PER_RUN,
            );
            let elapsed = start.elapsed();
            builder_durations_micros.push(elapsed.as_micros());
            println!(
                "(builder arguments run)  elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }
    }

    println!("\nAgregated results: (my_func never inlined)");
    println!(
        "(default arguments)  mean runtime: {:.3}ms, std_dev: {:.3}ms",
        mean(&default_durations_micros) / 1000.0,
        std_deviation(&default_durations_micros) / 1000.0
    );
    println!(
        "(explicit arguments) mean runtime: {:.3}ms, std_dev: {:.3}ms",
        mean(&explicit_durations_micros) / 1000.0,
        std_deviation(&explicit_durations_micros) / 1000.0
    );
    println!(
        "(builder arguments)  mean runtime: {:.3}ms, std_dev: {:.3}ms",
        mean(&builder_durations_micros) / 1000.0,
        std_deviation(&builder_durations_micros) / 1000.0
    );
}

fn main() {
    execute_my_func_benchmark();
    execute_my_func_never_inlined_benchmark();
}

fn mean(values: &[u128]) -> f32 {
    values.iter().sum::<u128>() as f32 / values.len() as f32
}

fn std_deviation(values: &[u128]) -> f32 {
    let mean = mean(values);
    let variance = values
        .iter()
        .map(|value| {
            let diff = mean - (*value as f32);
            diff * diff
        })
        .sum::<f32>()
        / values.len() as f32;
    variance.sqrt()
}
