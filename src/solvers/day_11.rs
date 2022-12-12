// Link - https://adventofcode.com/2022/day/11
use std::convert::From;

use crate::Solver;
use Operation::*;
use Unary::*;

pub struct Day11;

impl Solver for Day11 {
    fn part_a(&self, input: &str) -> String {
        let mut monkeys = parse_monkeys(input);

        let modulo: usize = monkeys.iter().map(|m| m.div).product();
        let mut bags = vec![vec![]; monkeys.len()];
        for _ in 0..20 {
            monkeys.iter_mut().enumerate().for_each(|(i, m)| {
                m.items.append(&mut bags[i]);
                m.items.drain(0..).for_each(|mut item| {
                    item = m.op.eval(item) / 3 % modulo;
                    bags[if item % m.div == 0 { m.yes } else { m.no }].push(item);
                    m.ins += 1;
                });
            });
        }

        monkeys.sort_by(|m1, m2| m2.ins.cmp(&m1.ins));
        (monkeys[0].ins * monkeys[1].ins).to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let mut monkeys = parse_monkeys(input);

        let modulo: usize = monkeys.iter().map(|m| m.div).product();
        let mut bags = vec![vec![]; monkeys.len()];
        for _ in 0..10000 {
            monkeys.iter_mut().enumerate().for_each(|(i, m)| {
                m.items.append(&mut bags[i]);
                m.items.drain(0..).for_each(|mut item| {
                    item = m.op.eval(item) % modulo;
                    bags[if item % m.div == 0 { m.yes } else { m.no }].push(item);
                    m.ins += 1;
                });
            });
        }

        monkeys.sort_by(|m1, m2| m2.ins.cmp(&m1.ins));
        (monkeys[0].ins * monkeys[1].ins).to_string()
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Binary,
    div: usize,
    yes: usize,
    no: usize,
    ins: usize,
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Unary {
    Literal(usize),
    Old,
}

impl From<&str> for Unary {
    fn from(s: &str) -> Self {
        match s {
            "old" => Old,
            val => Literal(val.trim().parse().unwrap()),
        }
    }
}

impl Unary {
    fn eval(&self, old: usize) -> usize {
        match self {
            Old => old,
            Literal(v) => *v,
        }
    }
}

#[derive(Debug)]
struct Binary {
    left: Unary,
    op: Operation,
    right: Unary,
}

impl From<&str> for Binary {
    fn from(s: &str) -> Self {
        let s: Vec<_> = s.split_ascii_whitespace().collect();

        let left = Unary::from(s[0]);
        let op = Operation::from(s[1]);
        let right = Unary::from(s[2]);

        Binary { left, op, right }
    }
}

impl Binary {
    fn eval(&self, old: usize) -> usize {
        let left = self.left.eval(old);
        let right = self.right.eval(old);

        match self.op {
            Add => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|grp| {
            let l: Vec<_> = grp.lines().map(|l| l.split(':').last().unwrap()).collect();
            let items = l[1].split(',').map(|i| i.trim().parse().unwrap()).collect();

            let op = Binary::from(l[2].split_once('=').unwrap().1);
            let div: usize = l[3]
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let yes: usize = l[4]
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let no: usize = l[5]
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                items,
                op,
                div,
                yes,
                no,
                ins: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(11);
        assert_eq!("10605", Day11.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(11);
        assert_eq!("2713310158", Day11.part_b(&input))
    }
}
