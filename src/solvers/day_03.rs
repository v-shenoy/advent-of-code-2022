// Link - https://adventofcode.com/2022/day/3
#![allow(unused)]
use std::collections::HashSet;

use std::convert::From;
use std::ops::{BitAnd, Deref};

use crate::Solver;

pub struct Day03;

impl Solver for Day03 {
    fn part_a(&self, input: &str) -> String {
        let ans: u64 = input
            .lines()
            .map(|l| {
                // Not the best solution due to the inefficiency of the HashSet.
                // Takes about 350μs.
                // let (first, second) = l.split_at(l.len() / 2);
                // let first: HashSet<char> = HashSet::from_iter(first.chars());
                // let second: HashSet<char> = HashSet::from_iter(second.chars());

                // let c = first.intersection(&second).next().unwrap();
                // priority(*c)

                // Efficient solution using a custom bit set. Uses bit indices to keep track of intersection.
                // The index for a character within the bitset is same as its priority.
                // Takes about 25μs.
                let (first, second) = l.split_at(l.len() / 2);
                let first = BitSet::from(first);
                let second = BitSet::from(second);

                let intersection = first & second;

                intersection.0.trailing_zeros() as u64
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
                // Read comments above.
                // let first: HashSet<char> = HashSet::from_iter(chunk[0].chars());
                // let second: HashSet<char> = HashSet::from_iter(chunk[1].chars());
                // let third: HashSet<char> = HashSet::from_iter(chunk[2].chars());

                // let mut intersection = first;
                // intersection.retain(|c| second.contains(c));
                // intersection.retain(|c| third.contains(c));

                // let c = intersection.iter().next().unwrap();

                // priority(*c)

                let intersection = chunk
                    .iter()
                    .map(|&s| BitSet::from(s))
                    .reduce(|acc, bitset| acc & bitset)
                    .unwrap();

                intersection.0.trailing_zeros() as u64
            })
            .sum();
        ans.to_string()
    }
}

struct BitSet(u64);

impl From<&str> for BitSet {
    fn from(s: &str) -> Self {
        let mut bitset = 0;
        s.chars().for_each(|c| bitset |= 1 << priority(c));

        BitSet(bitset)
    }
}

impl BitAnd for BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitSet(self.0 & rhs.0)
    }
}

// Priority for a character. Also used as the bit index for the character within a bitset.
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
        assert_eq!("157", Day03.part_a(&input));
    }

    #[test]
    fn test_part_b() {
        let input = read_input(3);
        assert_eq!("70", Day03.part_b(&input));
    }
}
