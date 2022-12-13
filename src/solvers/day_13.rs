// Link - https://adventofcode.com/2022/day/13
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::str::from_utf8;

use crate::Solver;
use Packet::*;

pub struct Day13;

impl Solver for Day13 {
    fn part_a(&self, input: &str) -> String {
        let mut ans = 0;

        input.split("\n\n").enumerate().for_each(|(i, l)| {
            let mut l = l.lines();
            let (left, right) = (
                parse_packet(l.next().unwrap().as_bytes(), &mut 0),
                parse_packet(l.next().unwrap().as_bytes(), &mut 0),
            );

            if left <= right {
                ans += i + 1;
            }
        });

        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let (divider_start, divider_end) = (
            List(vec![List(vec![Int(2)])]),
            List(vec![List(vec![Int(6)])]),
        );

        let mut packets = vec![divider_start.clone(), divider_end.clone()];

        input.split("\n\n").for_each(|l| {
            let mut l = l.lines();
            let (left, right) = (
                parse_packet(l.next().unwrap().as_bytes(), &mut 0),
                parse_packet(l.next().unwrap().as_bytes(), &mut 0),
            );

            packets.push(left);
            packets.push(right);
        });

        packets.sort_by(|p1, p2| p1.partial_cmp(p2).unwrap());

        let (mut idx_start, mut idx_end) = (0, 0);
        for (i, p) in packets.iter().enumerate() {
            if *p == divider_start {
                idx_start = i + 1;
            }

            if *p == divider_end {
                idx_end = i + 1;
            }
        }

        (idx_start * idx_end).to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
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

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Int(v1), Int(v2)) => Some(v1.cmp(v2)),
            (Int(v), List(_)) => List(vec![Int(*v)]).partial_cmp(other),
            (List(_), Int(v)) => self.partial_cmp(&List(vec![Int(*v)])),
            (List(packets1), List(packets2)) => {
                for (p1, p2) in packets1.iter().zip(packets2.iter()) {
                    let cmp = p1.partial_cmp(p2);
                    if let Some(Ordering::Equal) = cmp {
                        continue;
                    }

                    return cmp;
                }

                Some(packets1.len().cmp(&packets2.len()))
            }
        }
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
