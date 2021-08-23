use std::env;
use criterion::black_box;
use mylib::*;

fn main() {
    let arg = env::args().last().unwrap();
    let repetitions = black_box(REPETITIONS_PER_RUN);
    let result = match &arg[..] {
        "builder" => {
            builder_arguments_pattern::exercise_my_func_calls(repetitions)
        },
        "builder_never_inlined" => {
            builder_arguments_pattern::exercise_my_func_never_inlined_calls(repetitions)
        },
        "default" => {
            default_struct_arguments_pattern::exercise_my_func_calls(repetitions)
        },
        "default_never_inlined" => {
            use default_struct_arguments_pattern::exercise_my_func_never_inlined_calls;
            exercise_my_func_never_inlined_calls(repetitions)
        },
        "explicit" => {
            explicit_arguments_pattern::exercise_my_func_calls(repetitions)
        },
        "explicit_never_inlined" => {
            explicit_arguments_pattern::exercise_my_func_never_inlined_calls(repetitions)
        },
        _ => panic!("Invalid arg"),
    };
    black_box(result);
}
