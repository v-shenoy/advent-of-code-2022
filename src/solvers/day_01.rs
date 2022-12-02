// Link - https://adventofcode.com/2022/day/1
use crate::common::Solver;

pub struct Day01;

impl Solver for Day01 {
    fn part_a(&self, input: &str) -> String {
        let calories = calories_per_elf(input);
        calories.iter().max().unwrap().to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let mut calories = calories_per_elf(input);
        calories.sort();

        let ans: u64 = calories.iter().rev().take(3).sum();
        ans.to_string()
    }
}

fn calories_per_elf(input: &str) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u64>().unwrap()).sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input("inputs/01_test.txt");
        assert_eq!("24000", Day01.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input("inputs/01_test.txt");
        assert_eq!("45000", Day01.part_b(&input))
    }
}
