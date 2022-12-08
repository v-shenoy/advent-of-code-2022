// Link - https://adventofcode.com/2022/day/6
#![allow(unused)]
use std::collections::HashSet;

use crate::Solver;

pub struct Day06;

impl Solver for Day06 {
    fn part_a(&self, input: &str) -> String {
        const NUM_UNIQUE_CHARS: usize = 4;

        let ans = input
            .as_bytes()
            .windows(NUM_UNIQUE_CHARS)
            // Initial approach. Inefficient due to use of HashSet.
            // .position(|chars| chars.iter().collect::<HashSet<_>>().len() == NUM_UNIQUE_CHARS)
            // Found https://stackoverflow.com/a/46766782/653173 on Reddit, thanks to u/asaaki.
            .position(|chars| (1..chars.len()).all(|i| !chars[0..i].contains(&chars[i])))
            .unwrap()
            + NUM_UNIQUE_CHARS;

        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        const NUM_UNIQUE_CHARS: usize = 14;

        let ans = input
            .as_bytes()
            .windows(NUM_UNIQUE_CHARS)
            .position(|chars| (1..chars.len()).all(|i| !chars[0..i].contains(&chars[i])))
            .unwrap()
            + NUM_UNIQUE_CHARS;

        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(6);
        assert_eq!("7", Day06.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(6);
        assert_eq!("19", Day06.part_b(&input))
    }
}
