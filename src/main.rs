use clap::{Parser, Subcommand};
use std::time::Instant;

use aoc::{nano_to_micro, read_input, solvers};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Run the solution for particular part(s) for a particular day")]
    Run { day: u8, part: Option<char> },
    #[command(about = "Run solutions for all days and all parts")]
    All,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::All => {
            for (day, solver) in solvers::SOLVERS.iter().enumerate() {
                let day = day as u8 + 1;
                let input = read_input(day);

                let start = Instant::now();
                let ans = solver.part_a(&input);
                let elapsed = nano_to_micro(start.elapsed().as_nanos());
                println!(
                    "  Day - {:02}, Part A ans. - {}, Time - {}\u{00B5}s",
                    day, ans, elapsed
                );

                let start = Instant::now();
                let ans = solver.part_b(&input);
                let elapsed = nano_to_micro(start.elapsed().as_nanos());
                println!(
                    "  Day - {:02}, Part B ans. - {}, Time - {}\u{00B5}s",
                    day, ans, elapsed
                );

                println!("\u{2570} {} \u{256F}", "-".repeat(75));
            }
        }
        Command::Run { day, part } => {
            let solver = match solvers::SOLVERS.get((day - 1) as usize) {
                Some(solver) => solver,
                None => panic!("No solution for day - {:02}", day),
            };
            let input = read_input(day);

            match part {
                Some(part) => {
                    let part = part.to_uppercase().to_string();
                    let start = Instant::now();
                    let ans = match part.as_str() {
                        "A" => solver.part_a(&input),
                        "B" => solver.part_b(&input),
                        _ => panic!("No solution for Day - {:02}, Part {}", day, part),
                    };
                    let elapsed = nano_to_micro(start.elapsed().as_nanos());
                    println!(
                        "Day - {:02}, Part {} ans. - {}, Time - {}\u{00B5}s",
                        day, part, ans, elapsed
                    );
                }
                None => {
                    let start = Instant::now();
                    let ans = solver.part_a(&input);
                    let elapsed = nano_to_micro(start.elapsed().as_nanos());
                    println!(
                        "Day - {:02}, Part A ans. - {}, Time - {}\u{00B5}s",
                        day, ans, elapsed
                    );

                    let start = Instant::now();
                    let ans = solver.part_b(&input);
                    let elapsed = nano_to_micro(start.elapsed().as_nanos());
                    println!(
                        "Day - {:02}, Part B ans. - {}, Time - {}\u{00B5}s",
                        day, ans, elapsed
                    );
                }
            }
        }
    }
}
