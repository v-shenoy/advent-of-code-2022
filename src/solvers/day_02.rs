// Link - https://adventofcode.com/2022/day/2
use std::convert::From;

use crate::Solver;
use Choice::*;
use Outcome::*;

pub struct Day02;

impl Solver for Day02 {
    fn part_a(&self, input: &str) -> String {
        let ans: u64 = input
            .lines()
            .map(|l| {
                let (opp, your) = l.split_once(' ').unwrap();
                let opp = Choice::from(opp);
                let your = Choice::from(your);

                (your.value() + your.outcome(&opp).value()) as u64
            })
            .sum();

        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let ans: u64 = input
            .lines()
            .map(|l| {
                let (opp, res) = l.split_once(' ').unwrap();
                let opp = Choice::from(opp);
                let res = Outcome::from(res);

                let your = match (opp, res) {
                    (Rock, Loss) => Scissors,
                    (Rock, Draw) => Rock,
                    (Rock, Win) => Paper,
                    (Paper, Loss) => Rock,
                    (Paper, Draw) => Paper,
                    (Paper, Win) => Scissors,
                    (Scissors, Loss) => Paper,
                    (Scissors, Draw) => Scissors,
                    (Scissors, Win) => Rock,
                };

                (your.value() + res.value()) as u64
            })
            .sum();

        ans.to_string()
    }
}

#[derive(Copy, Clone, Debug)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Choice {
    fn from(choice: &str) -> Self {
        match choice {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!("Invalid choice - {}", choice),
        }
    }
}

impl Choice {
    fn value(&self) -> u8 {
        *self as u8
    }

    fn outcome(&self, opp: &Choice) -> Outcome {
        match (self, opp) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Loss,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(res: &str) -> Self {
        match res {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!("Invalid Outcome - {}", res),
        }
    }
}

impl Outcome {
    fn value(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(2);
        assert_eq!("15", Day02.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(2);
        assert_eq!("12", Day02.part_b(&input))
    }
}
