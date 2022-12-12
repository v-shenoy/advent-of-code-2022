# :christmas_tree:advent-of-code-2022

## Overview

This repository contains my solutions to [Advent of Code 2022](https://adventofcode.com/2022).

I decided to take this year as an opportunity to learn Rust. My goal will be to write simple, readable, and idiomatic Rust code as much as I can.
I will be restricting myself to the standard library and try to get familiar with it.

I will also try to add basic comments explaining my approach, as well as benchmark the code.

> Note - This is my first time writing Rust programs. For many of my solutions there might be a better way,
> in terms of efficiency or writing Rust-y code.

## Solutions

All common utility stuff is defined in the `src/lib.rs` file. Solutions for day `XY` are defined in the `src/solvers/day_XY.rs` file. Inputs
for day `XY` are in the `inputs/XY.txt` and `inputs/XY_test.txt` (which is the sample input).

> Note:
> - The solutions and tests been checked with Cargo & Rust v1.65.0.
> - The benchmarks been checked with Cargo & Rust v1.67.0-nightly.

### Run

```shell
# Running all solutions
$ cargo run --release -- all
# Running a particular solution
$ cargo run --release -- run <day> [part] # day = XY, part = a / b (optional, runs both parts by default)
```

### Test

Sample input for each day has been used to write unit tests.

```shell
# Testing all sample inputs
$ cargo test
# Testing sample input for particular day
$ cargo test XY
```

### Benchmarks

> Note: This requires features from the unstable release chain, which can be installed using -
> ```shell
> $ rustup install nightly
> ```

Once nightly rust has been installed
```shell
# Running all benchmarks
$ cargo +nightly bench
# Running benchmarks for particular day
$ cargo +nightly bench XY
```

## Stars obtained

### 24 / 50 :star:

## Other Advent of Code solutions

| Year | Repo | Language |
| ---- | ---- | -------- |
| 2021 | [advent-of-code-2021](https://github.com/v-shenoy/advent-of-code-2021) | Python |
| 2022 | [advent-of-code-2022](https://github.com/v-shenoy/advent-of-code-2022) (this repo) | Rust |


## Acknowledgements

Thanks to [Eric Wastl](https://github.com/topaz) for creating and managing the Advent of Code project.
