use crate::Solver;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_xy;

pub const SOLVERS: [&dyn Solver; 5] = [
    &day_01::Day01,
    &day_02::Day02,
    &day_03::Day03,
    &day_04::Day04,
    &day_05::Day05,
];
