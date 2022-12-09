use std::fs;

pub mod solvers;

pub fn read_input(day: u8) -> String {
    let path = format!(
        "inputs/{}{day:02}.txt",
        if cfg!(test) { "tests/" } else { "" },
    );
    match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(err) => panic!("Error reading input file {path} - {err}"),
    }
}

const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];
pub fn format_time(time: u128) -> String {
    let mut time = time as f64;
    for unit in TIME_UNITS {
        if time < 1000_f64 {
            return format!("{time}{unit}");
        }
        time /= 1000_f64;
    }

    format!("{time}{}", TIME_UNITS.last().unwrap())
}

pub trait Solver {
    fn part_a(&self, input: &str) -> String;
    fn part_b(&self, input: &str) -> String;
}
