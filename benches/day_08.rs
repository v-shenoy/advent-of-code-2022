#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_08::Day08, Solver};

mod day_08 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(8);
        b.iter(|| Day08.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(8);
        b.iter(|| Day08.part_b(&input));
    }
}
