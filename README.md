## Description

This is a simple benchmark to evaluate the performance impact of using structs that implement Default as function arguments compared against the regular corresponding implementation using explicit arguments.

## How to run the benchmark

It's important to run the benchmark in release mode, to allow the compiler to do its optimization magic. Otherwise the default arguments implementation gets substantially penalized.

```
cargo run --release
```

## Sample result obtained from a run in my computer

The performance of the inlined my_func implementations initially seemed indistinguishable when running in my CPU:

```
Agregated results: (my_func inlined)
(default arguments)  mean runtime: 59.839ms, std_dev: 1.892ms
(explicit arguments) mean runtime: 59.886ms, std_dev: 1.757ms
(builder arguments)  mean runtime: 59.868ms, std_dev: 2.238ms
```

If we prevent the functions to get inlined though (by adding ```#[inline(never)]``` before the ```my_func``` and ```my_func2``` definitions), the default arguments and builder implementations performances get heavily impacted:

```
Agregated results: (my_func never inlined)
(default arguments)  mean runtime: 832.429ms, std_dev: 37.043ms
(explicit arguments) mean runtime: 192.341ms, std_dev: 8.229ms
(builder arguments)  mean runtime: 825.382ms, std_dev: 32.820ms
```
