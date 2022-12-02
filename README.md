# :christmas_tree:advent-of-code-2022

## Overview

This repository contains my solutions to [Advent of Code 2022](https://adventofcode.com/2022).

I decided to take this year as an opportunity to learn Rust. My goal will be to write simple, readable, and idiomatic Rust code as much as I can.
I will be restricting myself to the standard library and try to get familiar with it.

I will also try to add basic comments explaining my approach, as well as benchmark the code.

## Solutions

All common utility stuff is defined in the `src/common.rs` file. Solutions for day `XX` are defined in the `src/solvers/day_XX.rs` file. Inputs
for day `XX` are in the `inputs/XX.txt` and `inputs/XX_test.txt` (which is the sample input).

### Using Cargo

```shell
# Running the sample inputs
$ cargo test
# Running all solutions
$ cargo run --release -- all
# Running a particular solution
$ cargo run --release -- run <day> <part> # day = XX, part = a/b
```

> Note: The solutions have been tested with Cargo & Rust v1.65.0.

## Stars obtained

### 02 / 50 :star:

## Other Advent of Code solutions

| Year | Repo | Language |
| ---- | ---- | -------- |
| 2021 | [advent-of-code-2021](https://github.com/v-shenoy/advent-of-code-2021) | Python |
| 2022 | [advent-of-code-2022](https://github.com/v-shenoy/advent-of-code-2022) (this repo) | Rust |


## Acknowledgements

Thanks to [Eric Wastl](https://github.com/topaz) for creating and managing the Advent of Code project.
