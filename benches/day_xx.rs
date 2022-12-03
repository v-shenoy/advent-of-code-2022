#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_xx::DayXX, Solver};

mod day_xx {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| DayXX.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| DayXX.part_b(&input));
    }
}
