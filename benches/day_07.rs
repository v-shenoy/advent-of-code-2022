#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_07::Day07, Solver};

mod day_07 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(7);
        b.iter(|| Day07.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(7);
        b.iter(|| Day07.part_b(&input));
    }
}
