// Link - https://adventofcode.com/2022/day/9
use std::{collections::HashSet, convert::From};

use crate::Solver;
use Direction::*;

pub struct Day09;

impl Solver for Day09 {
    fn part_a(&self, input: &str) -> String {
        const ROPE_LEN: usize = 2;
        let mut rope = Rope::new(ROPE_LEN);
        let mut visited = HashSet::new();

        visited.insert(rope.end());
        input.lines().for_each(|l| {
            let (dir, step) = l.split_once(' ').unwrap();
            let dir = Direction::from(dir);
            let step: i64 = step.parse().unwrap();

            rope.pull(dir, step, &mut visited);
        });

        visited.len().to_string()
    }

    fn part_b(&self, input: &str) -> String {
        const ROPE_LEN: usize = 10;
        let mut rope = Rope::new(ROPE_LEN);
        let mut visited = HashSet::new();

        visited.insert(rope.end());
        input.lines().for_each(|l| {
            let (dir, step) = l.split_once(' ').unwrap();
            let dir = Direction::from(dir);
            let step: i64 = step.parse().unwrap();

            rope.pull(dir, step, &mut visited);
        });

        visited.len().to_string()
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Down,
    Right,
    Up,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Left,
            "D" => Down,
            "R" => Right,
            "U" => Up,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Knot {
    x: i64,
    y: i64,
}

impl Knot {
    fn new(x: i64, y: i64) -> Self {
        Knot { x, y }
    }
}

#[derive(Debug)]
struct Rope {
    len: usize,
    knots: Vec<Knot>,
}

impl Rope {
    fn new(len: usize) -> Self {
        Rope {
            len,
            knots: vec![Knot::new(0, 0); len],
        }
    }

    fn pull(&mut self, dir: Direction, step: i64, visited: &mut HashSet<Knot>) {
        for _ in 0..step {
            match dir {
                Left => self.knots[0].x -= 1,
                Down => self.knots[0].y -= 1,
                Right => self.knots[0].x += 1,
                Up => self.knots[0].y += 1,
            }
            self.update_knots(visited);
        }
    }

    fn update_knots(&mut self, visited: &mut HashSet<Knot>) {
        for i in 1..self.len {
            let prev = self.knots[i - 1];
            let next = self.knots[i];

            let (dx, dy) = (prev.x - next.x, prev.y - next.y);
            if dx.abs() > 1 || dy.abs() > 1 {
                self.knots[i].x += dx.signum();
                self.knots[i].y += dy.signum();
            }
        }

        visited.insert(self.end());
    }

    fn end(&self) -> Knot {
        *self.knots.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(9);
        assert_eq!("13", Day09.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(9);
        assert_eq!("1", Day09.part_b(&input))
    }
}
