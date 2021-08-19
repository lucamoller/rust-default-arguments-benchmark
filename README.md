## Description

This is a simple benchmark to evaluate the performance impact of using structs that implement Default as function arguments compared against the regular corresponding implementation using explicit arguments.

## How to run the benchmark

It's important to run the benchmark in release mode, to allow the compiler to do its optimizations magic. Otherwise the default arguments implementation gets substantially penalized.

```
cargo run --release
```

## Sample result obtained from a run in my computer

The performance of the two implementations initially seemed indistinguishable when running in my CPU.

```
(default arguments)  mean runtime: 59.268ms, std_dev: 3.343ms
(explicit arguments) mean runtime: 59.351ms, std_dev: 3.340ms
```

If we prevent the functions to get inlined though (by adding ```#[inline(never)]``` before the ```my_func``` and ```my_func2``` definitions), the default arguments implementation performance gets heavily impacted:

```
(default arguments)  mean runtime: 883.652ms, std_dev: 31.416ms
(explicit arguments) mean runtime: 167.352ms, std_dev: 7.446ms
```