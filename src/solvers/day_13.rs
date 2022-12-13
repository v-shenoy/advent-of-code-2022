// Link - https://adventofcode.com/2022/day/13
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::str::from_utf8;

use crate::Solver;
use Packet::*;

pub struct Day13;

impl Solver for Day13 {
    fn part_a(&self, input: &str) -> String {
        let mut ans = 0;

        input.split("\n\n").enumerate().for_each(|(i, l)| {
            let (left, right) = l.split_once('\n').unwrap();
            let (left, right) = (
                parse_packet(left.as_bytes(), &mut 0),
                parse_packet(right.as_bytes(), &mut 0),
            );

            if left <= right {
                ans += i + 1;
            }
        });

        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let mut packets: Vec<_> = input
            .lines()
            .filter_map(|l| {
                if !l.is_empty() {
                    Some(parse_packet(l.as_bytes(), &mut 0))
                } else {
                    None
                }
            })
            .collect();
        packets.sort();

        let idx_start = match packets.binary_search(&parse_packet("[[2]]".as_bytes(), &mut 0)) {
            Ok(i) => i,
            Err(i) => i,
        } + 1;
        let idx_end = match packets.binary_search(&parse_packet("[[6]]".as_bytes(), &mut 0)) {
            Ok(i) => i,
            Err(i) => i,
        } + 2;

        (idx_start * idx_end).to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(usize),
    List(Vec<Self>),
}

fn parse_packet(s: &[u8], idx: &mut usize) -> Packet {
    let mut packets = Vec::new();

    *idx += 1;
    while s[*idx] != b']' {
        if s[*idx].is_ascii_digit() {
            let start = *idx;
            while s[*idx].is_ascii_digit() {
                *idx += 1;
            }

            let val: usize = from_utf8(&s[start..*idx]).unwrap().parse().unwrap();
            packets.push(Int(val))
        } else if s[*idx] == b',' {
            *idx += 1;
        } else if s[*idx] == b'[' {
            packets.push(parse_packet(s, idx));
        }
    }
    *idx += 1;

    List(packets)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(v1), Int(v2)) => v1.cmp(v2),
            (Int(v), List(_)) => List(vec![Int(*v)]).cmp(other),
            (List(_), Int(v)) => self.cmp(&List(vec![Int(*v)])),
            (List(packets1), List(packets2)) => packets1.cmp(packets2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(13);
        assert_eq!("13", Day13.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(13);
        assert_eq!("140", Day13.part_b(&input))
    }
}
