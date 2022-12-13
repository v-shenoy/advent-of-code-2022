use crate::Solver;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_xy;

pub const SOLVERS: [&dyn Solver; 13] = [
    &day_01::Day01,
    &day_02::Day02,
    &day_03::Day03,
    &day_04::Day04,
    &day_05::Day05,
    &day_06::Day06,
    &day_07::Day07,
    &day_08::Day08,
    &day_09::Day09,
    &day_10::Day10,
    &day_11::Day11,
    &day_12::Day12,
    &day_13::Day13,
];
