#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_03::Day03, Solver};

mod day_03 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(3);
        b.iter(|| Day03.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(3);
        b.iter(|| Day03.part_b(&input));
    }
}
