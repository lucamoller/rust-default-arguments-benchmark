pub extern crate criterion;

pub mod common;
pub mod builder_arguments_pattern;
pub mod default_struct_arguments_pattern;
pub mod explicit_arguments_pattern;

pub const REPETITIONS_PER_RUN: i64 = 100000000;
