#![feature(test)]
extern crate test;
use test::Bencher;

#[path = "../src/common.rs"]
mod common;
use common::{read_input, Solver};

#[path = "../src/solvers/day_02.rs"]
mod day_02;
use day_02::Day02;

mod bench_day_02 {
    use super::*;

    #[bench]
    fn bench_part_a(b: &mut Bencher) {
        let input = read_input("inputs/02.txt");
        b.iter(|| Day02.part_a(&input));
    }

    #[bench]
    fn bench_part_b(b: &mut Bencher) {
        let input = read_input("inputs/02.txt");
        b.iter(|| Day02.part_b(&input));
    }
}
