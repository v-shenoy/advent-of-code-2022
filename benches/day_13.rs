#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_13::Day13, Solver};

mod day_13 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(13);
        b.iter(|| Day13.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(13);
        b.iter(|| Day13.part_b(&input));
    }
}
