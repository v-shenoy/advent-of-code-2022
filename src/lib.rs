use std::fs;

pub mod solvers;

pub fn read_input(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(input) => input,
        Err(err) => panic!("Error reading input file {} - {}", path, err),
    }
}

pub fn nano_to_micro(ns: u128) -> f64 {
    ns as f64 / 1_000_f64
}

pub trait Solver {
    fn part_a(&self, input: &str) -> String;
    fn part_b(&self, input: &str) -> String;
}
