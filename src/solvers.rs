use crate::Solver;

mod day_01;
mod day_02;

pub const SOLVERS: [&dyn Solver; 2] = [&day_01::Day01, &day_02::Day02];
