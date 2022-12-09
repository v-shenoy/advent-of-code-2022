#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_09::Day09, Solver};

mod day_09 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(9);
        b.iter(|| Day09.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(9);
        b.iter(|| Day09.part_b(&input));
    }
}
