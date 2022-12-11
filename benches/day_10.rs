#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_10::Day10, Solver};

mod day_10 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(10);
        b.iter(|| Day10.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(10);
        b.iter(|| Day10.part_b(&input));
    }
}
