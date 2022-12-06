#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_06::Day06, Solver};

mod day_06 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(6);
        b.iter(|| Day06.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(6);
        b.iter(|| Day06.part_b(&input));
    }
}
