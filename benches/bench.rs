use std::stringify;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mylib::*;

const REPETITIONS_PER_RUN: i64 = 100000000;

macro_rules! register {
    ( $f:expr ) => {
        ($f as fn(i64) -> bool, stringify!($f))
    }
}

fn bench_patterns(c: &mut Criterion) {
    let funcs = [
        register!(default_struct_arguments_pattern::exercise_my_func_calls),
        register!(default_struct_arguments_pattern::exercise_my_func_never_inlined_calls),
        register!(builder_arguments_pattern::exercise_my_func_calls),
        register!(builder_arguments_pattern::exercise_my_func_never_inlined_calls),
        register!(explicit_arguments_pattern::exercise_my_func_calls),
        register!(explicit_arguments_pattern::exercise_my_func_never_inlined_calls),
    ];
    for (f, name) in funcs {
        c.bench_function(
            name,
            |b| b.iter(|| {
                black_box( f(black_box(REPETITIONS_PER_RUN)) );
            })
        );
    }
}

criterion_group!(benches, bench_patterns);
criterion_main!(benches);
