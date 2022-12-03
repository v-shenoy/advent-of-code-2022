// Link - https://adventofcode.com/2022/day/3
use std::collections::HashSet;

use crate::Solver;

pub struct Day03;

impl Solver for Day03 {
    fn part_a(&self, input: &str) -> String {
        let ans: u64 = input
            .lines()
            .map(|l| {
                let (first, second) = l.split_at(l.len() / 2);
                let first: HashSet<char> = HashSet::from_iter(first.chars());
                let second: HashSet<char> = HashSet::from_iter(second.chars());

                let c = first.intersection(&second).next().unwrap();

                priority(*c)
            })
            .sum();
        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let ans: u64 = input
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| {
                let first: HashSet<char> = HashSet::from_iter(chunk[0].chars());
                let second: HashSet<char> = HashSet::from_iter(chunk[1].chars());
                let third: HashSet<char> = HashSet::from_iter(chunk[2].chars());

                let mut intersection = first;
                intersection.retain(|c| second.contains(c));
                intersection.retain(|c| third.contains(c));

                let c = intersection.iter().next().unwrap();

                priority(*c)
            })
            .sum();
        ans.to_string()
    }
}

fn priority(c: char) -> u64 {
    if c.is_ascii_uppercase() {
        return (c as u64) - ('A' as u64) + 27;
    }

    (c as u64) - ('a' as u64) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(3);
        assert_eq!("157", Day03.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(3);
        assert_eq!("70", Day03.part_b(&input))
    }
}
