use crate::Solver;

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub const SOLVERS: [&dyn Solver; 3] = [&day_01::Day01, &day_02::Day02, &day_03::Day03];
