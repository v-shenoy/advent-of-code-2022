#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_04::Day04, Solver};

mod day_04 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| Day04.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| Day04.part_b(&input));
    }
}
