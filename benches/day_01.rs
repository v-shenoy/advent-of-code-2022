#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_01::Day01, Solver};

mod day_01 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input("inputs/01.txt");
        b.iter(|| Day01.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input("inputs/01.txt");
        b.iter(|| Day01.part_b(&input));
    }
}
