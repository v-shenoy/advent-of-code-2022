#![feature(test)]
extern crate test;
use test::Bencher;

use aoc::{read_input, solvers::day_xy::DayXY, Solver};

mod day_xy {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| DayXY.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input(4);
        b.iter(|| DayXY.part_b(&input));
    }
}
