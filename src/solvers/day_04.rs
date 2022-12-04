// Link - https://adventofcode.com/2022/day/4
use crate::Solver;
use std::convert::From;

pub struct Day04;

impl Solver for Day04 {
    fn part_a(&self, input: &str) -> String {
        let ans = input
            .lines()
            .map(|l| {
                let (first, other) = l.split_once(',').unwrap();
                (Range::from(first), Range::from(other))
            })
            .filter(|(first, other)| first.contains(other) || other.contains(first))
            .count();
        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let ans = input
            .lines()
            .map(|l| {
                let (first, other) = l.split_once(',').unwrap();
                (Range::from(first), Range::from(other))
            })
            .filter(|(first, other)| first.intersects(other))
            .count();
        ans.to_string()
    }
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn intersects(&self, other: &Range) -> bool {
        other.end >= self.start && other.start <= self.end
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        Range { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(4);
        assert_eq!("2", Day04.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(4);
        assert_eq!("4", Day04.part_b(&input))
    }
}
