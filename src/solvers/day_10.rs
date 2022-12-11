// Link - https://adventofcode.com/2022/day/10
use std::convert::From;

use crate::Solver;
use Instruction::*;

pub struct Day10;

impl Solver for Day10 {
    fn part_a(&self, input: &str) -> String {
        let program: Vec<Instruction> = input.lines().map(Instruction::from).collect();

        let cpu = Cpu::new();
        cpu.execute_program(program.as_slice()).to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let program: Vec<Instruction> = input.lines().map(Instruction::from).collect();

        let cpu = Cpu::new();
        cpu.draw(program.as_slice())
    }
}

enum Instruction {
    NoOp,
    Add(i64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s.split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {
            ["noop"] => NoOp,
            ["addx", v] => Add(v.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

struct Cpu {
    reg: i64,
    curr_cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            reg: 1,
            curr_cycle: 1,
        }
    }

    fn execute_program(mut self, program: &[Instruction]) -> i64 {
        let mut signal: i64 = 0;
        let mut next_cycle = 20;

        self.calculate_signal(&mut next_cycle, &mut signal);
        for instr in program {
            match instr {
                NoOp => {
                    self.curr_cycle += 1;
                    self.calculate_signal(&mut next_cycle, &mut signal);
                }
                Add(v) => {
                    self.curr_cycle += 1;
                    self.calculate_signal(&mut next_cycle, &mut signal);

                    self.curr_cycle += 1;
                    self.reg += v;
                    self.calculate_signal(&mut next_cycle, &mut signal);
                }
            }
        }

        signal
    }

    fn calculate_signal(&mut self, next_cycle: &mut usize, signal: &mut i64) {
        if self.curr_cycle == *next_cycle {
            *signal += self.reg * self.curr_cycle as i64;
            *next_cycle += 40;
        }
    }

    fn draw(mut self, program: &[Instruction]) -> String {
        let mut display = vec!['\n'];

        self.draw_sprite(&mut display);
        for instr in program {
            match instr {
                NoOp => {
                    self.curr_cycle += 1;
                    self.draw_sprite(&mut display);
                }
                Add(v) => {
                    self.curr_cycle += 1;
                    self.draw_sprite(&mut display);

                    self.curr_cycle += 1;
                    self.reg += v;
                    self.draw_sprite(&mut display);
                }
            }
        }

        display.iter().collect()
    }

    fn draw_sprite(&mut self, display: &mut Vec<char>) {
        let idx = ((self.curr_cycle - 1) % 40) as i64;

        if (self.reg - 1..=self.reg + 1).contains(&idx) {
            display.push('#');
        } else {
            display.push(' ');
        }

        if self.curr_cycle % 40 == 0 {
            display.push('\n');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(10);
        assert_eq!("13140", Day10.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(10);

        assert_eq!(
            "\n##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     \n ",
            Day10.part_b(&input)
        );
    }
}
