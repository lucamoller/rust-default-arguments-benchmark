use std::time::Instant;

pub struct MyFuncArgs<'a> {
    pub req1: &'a str,
    pub req2: &'a str,
    pub optional_args: MyFuncOptionalArgs<'a>,
}

pub struct MyFuncOptionalArgs<'a> {
    pub opt1: i32,
    pub opt2: &'a str,
    pub opt3: bool,
}

impl<'a> Default for MyFuncOptionalArgs<'a> {
    fn default() -> Self {
        MyFuncOptionalArgs {
            opt1: 123,
            opt2: "abc",
            opt3: false,
        }
    }
}

// #[inline(never)]
pub fn my_func(args: MyFuncArgs) -> bool {
    (args.req1.len() % 2 == 0)
        ^ (args.req2.len() % 2 == 0)
        ^ (args.optional_args.opt1 % 2 == 0)
        ^ (args.optional_args.opt2.len() % 2 == 0)
        ^ args.optional_args.opt3
}

// #[inline(never)]
pub fn my_func2(req1: &str, req2: &str, opt1: i32, opt2: &str, opt3: bool) -> bool {
    (req1.len() % 2 == 0) ^ (req2.len() % 2 == 0) ^ (opt1 % 2 == 0) ^ (opt2.len() % 2 == 0) ^ opt3
}

const N_RUNS: i64 = 20;
const N_LOOPS_PER_RUN: i64 = 100000000;

fn main() {
    let mut default_durations_micros = Vec::new();
    let mut explicit_durations_micros = Vec::new();

    for _ in 0..N_RUNS {
        // Intercalate default and explicit calls.
        {
            let start = Instant::now();
            let mut result = false;
            for i in 0..N_LOOPS_PER_RUN {
                result ^= my_func(MyFuncArgs {
                    req1: "req1value_call1",
                    req2: "req2value_call1",
                    optional_args: MyFuncOptionalArgs::default(),
                });
                result ^= my_func(MyFuncArgs {
                    req1: "req1value_call2",
                    req2: "req2value_call2",
                    optional_args: MyFuncOptionalArgs {
                        opt1: 456 + (i % 2) as i32,
                        opt2: "def",
                        opt3: true,
                        ..Default::default()
                    },
                });
                result ^= my_func(MyFuncArgs {
                    req1: "req1value_call3",
                    req2: "req2value_call3",
                    optional_args: MyFuncOptionalArgs {
                        opt2: "ghi",
                        ..Default::default()
                    },
                });
                result ^= my_func(MyFuncArgs {
                    req1: "req1value_call4",
                    req2: "req2value_call4",
                    optional_args: MyFuncOptionalArgs {
                        opt3: true,
                        opt1: 789 + (i % 3) as i32,
                        ..Default::default()
                    },
                });
            }
            let elapsed = start.elapsed();
            default_durations_micros.push(elapsed.as_micros());
            println!(
                "(default arguments run)  elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }
        {
            let start = Instant::now();
            let mut result = false;
            for i in 0..N_LOOPS_PER_RUN {
                result ^= my_func2("req1value_call1", "req2value_call1", 123, "abc", false);
                result ^= my_func2(
                    "req1value_call2",
                    "req2value_call2",
                    456 + (i % 2) as i32,
                    "def",
                    true,
                );
                result ^= my_func2("req1value_call3", "req2value_call3", 123, "ghi", false);
                result ^= my_func2(
                    "req1value_call4",
                    "req2value_call4",
                    789 + (i % 3) as i32,
                    "abc",
                    true,
                );
            }
            let elapsed = start.elapsed();
            explicit_durations_micros.push(elapsed.as_micros());
            println!(
                "(explicit arguments run) elapsed: {:.4?}, result: {}",
                elapsed, result
            );
        }
    }

    println!("\nAgregated results:");
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
