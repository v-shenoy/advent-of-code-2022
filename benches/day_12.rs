#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_12::Day12, Solver};

mod day_12 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(12);
        b.iter(|| Day12.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(12);
        b.iter(|| Day12.part_b(&input));
    }
}
