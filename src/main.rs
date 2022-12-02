use clap::{Parser, Subcommand};
use std::time::Instant;

mod common;
mod solvers;

use common::{nano_to_milli, read_input, Solver};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Run the solution for a particular part for a particular day")]
    Run { day: u8, part: Option<char> },
    #[command(about = "Run solutions for all days and all parts")]
    All,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::All => {
            for (day, solver) in solvers::SOLVERS.iter().enumerate() {
                let day = day as u8 + 1;
                let input_file = format!("inputs/{:02}.txt", day);
                let input = read_input(&input_file);

                let start = Instant::now();
                let ans = solver.part_a(&input);
                let elapsed = nano_to_milli(start.elapsed().as_nanos());
                println!(
                    "Day - {:02}, Part A ans. - {}, Time - {}ms",
                    day, ans, elapsed
                );

                let start = Instant::now();
                let ans = solver.part_b(&input);
                let elapsed = nano_to_milli(start.elapsed().as_nanos());
                println!(
                    "Day - {:02}, Part B ans. - {}, Time - {}ms",
                    day, ans, elapsed
                );

                println!("--");
            }
        }
        Command::Run { day, part } => match part {
            Some(part) => {
                let part = part.to_uppercase().to_string();

                let input_file = format!("inputs/{:02}.txt", day);
                let input = read_input(&input_file);
                let solver = match solvers::SOLVERS.get((day - 1) as usize) {
                    Some(solver) => solver,
                    None => panic!("No solution for day - {:02}", day),
                };

                let start = Instant::now();
                let ans = match part.as_str() {
                    "A" => solver.part_a(&input),
                    "B" => solver.part_b(&input),
                    _ => panic!("No solution for Day - {:02}, Part {}", day, part),
                };
                let elapsed = nano_to_milli(start.elapsed().as_nanos());
                println!(
                    "Day - {:02}, Part {} ans. - {}, Time - {}ms",
                    day, part, ans, elapsed
                );
            }
            None => {
                let input_file = format!("inputs/{:02}.txt", day);
                let input = read_input(&input_file);
                let solver = match solvers::SOLVERS.get((day - 1) as usize) {
                    Some(solver) => solver,
                    None => panic!("No solution for day - {:02}", day),
                };

                let start = Instant::now();
                let ans = solver.part_a(&input);
                let elapsed = nano_to_milli(start.elapsed().as_nanos());
                println!(
                    "Day - {:02}, Part A ans. - {}, Time - {}ms",
                    day, ans, elapsed
                );

                let start = Instant::now();
                let ans = solver.part_b(&input);
                let elapsed = nano_to_milli(start.elapsed().as_nanos());
                println!(
                    "Day - {:02}, Part B ans. - {}, Time - {}ms",
                    day, ans, elapsed
                );
            }
        },
    }
}
