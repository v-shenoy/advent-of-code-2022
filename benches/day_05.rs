#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_05::Day05, Solver};

mod day_05 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(5);
        b.iter(|| Day05.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(5);
        b.iter(|| Day05.part_b(&input));
    }
}
