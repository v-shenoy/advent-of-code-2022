use clap::{Parser, Subcommand, ValueEnum};
use std::{fmt::Display, time::Instant};

use aoc::{format_time, read_input, solvers};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Run the solution for particular part(s) for a particular day")]
    Run { day: u8, part: Option<Part> },
    #[command(about = "Run solutions for all days and all parts")]
    All,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    A,
    B,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::All => {
            let start = Instant::now();
            for (day, solver) in solvers::SOLVERS.iter().enumerate() {
                let day = day as u8 + 1;
                let input = read_input(day);

                let mut start = Instant::now();
                let mut ans = solver.part_a(&input);
                let mut elapsed = format_time(start.elapsed().as_nanos());
                println!("  Day - {day:02}, Part A ans. - {ans}, Time - {elapsed}",);

                start = Instant::now();
                ans = solver.part_b(&input);
                elapsed = format_time(start.elapsed().as_nanos());
                println!("  Day - {day:02}, Part B ans. - {ans}, Time - {elapsed}",);

                println!("╰ {} ╯", "-".repeat(75));
            }
            let elapsed = format_time(start.elapsed().as_nanos());
            println!("Overall time - {elapsed}");
        }
        Command::Run { day, part } => {
            let solver = match solvers::SOLVERS.get((day - 1) as usize) {
                Some(solver) => solver,
                None => panic!("No solution for day - {day:02}"),
            };
            let input = read_input(day);

            match part {
                Some(part) => {
                    let start = Instant::now();
                    let ans = match part {
                        Part::A => solver.part_a(&input),
                        Part::B => solver.part_b(&input),
                    };
                    let elapsed = format_time(start.elapsed().as_nanos());
                    println!("Day - {day:02}, Part {part} ans. - {ans}, Time - {elapsed}",);
                }
                None => {
                    let mut start = Instant::now();
                    let mut ans = solver.part_a(&input);
                    let mut elapsed = format_time(start.elapsed().as_nanos());
                    println!("Day - {day:02}, Part A ans. - {ans}, Time - {elapsed}",);

                    start = Instant::now();
                    ans = solver.part_b(&input);
                    elapsed = format_time(start.elapsed().as_nanos());
                    println!("Day - {day:02}, Part B ans. - {ans}, Time - {elapsed}",);
                }
            }
        }
    }
}
