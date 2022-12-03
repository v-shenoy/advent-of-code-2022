#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_02::Day02, Solver};

mod day_02 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(2);
        b.iter(|| Day02.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(2);
        b.iter(|| Day02.part_b(&input));
    }
}
