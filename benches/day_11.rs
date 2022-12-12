#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_11::Day11, Solver};

mod day_11 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(11);
        b.iter(|| Day11.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(11);
        b.iter(|| Day11.part_b(&input));
    }
}
