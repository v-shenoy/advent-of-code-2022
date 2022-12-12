// Link - https://adventofcode.com/2022/day/12
use std::{cmp::min, collections::VecDeque};

use crate::Solver;

pub struct Day12;

impl Solver for Day12 {
    fn part_a(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (rows, cols) = (grid.len(), grid[0].len());
        let heightmap = HeightMap { grid, rows, cols };

        let dist = heightmap.bfs();

        let (x, y) = heightmap.find_start();
        dist[x][y].to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (rows, cols) = (grid.len(), grid[0].len());
        let heightmap = HeightMap { grid, rows, cols };

        let (dist, mut min_dist) = (heightmap.bfs(), isize::MAX);
        for (x, row) in dist.iter().enumerate() {
            for (y, &d) in row.iter().enumerate() {
                if heightmap.is_lowest(x, y) && d != -1 {
                    min_dist = min(min_dist, d);
                }
            }
        }

        min_dist.to_string()
    }
}

struct HeightMap {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

type Index = (usize, usize);

impl HeightMap {
    fn find_start(&self) -> Index {
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.grid[x][y] == 'S' {
                    return (x, y);
                }
            }
        }

        unreachable!();
    }

    fn find_end(&self) -> Index {
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.grid[x][y] == 'E' {
                    return (x, y);
                }
            }
        }

        unreachable!();
    }

    fn can_move(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let src = if self.grid[x1][y1] == 'E' {
            b'z'
        } else {
            self.grid[x1][y1] as u8
        };

        let dest = if self.grid[x2][y2] == 'S' {
            b'a'
        } else {
            self.grid[x2][y2] as u8
        };

        dest + 1 >= src
    }

    fn is_lowest(&self, x: usize, y: usize) -> bool {
        self.grid[x][y] == 'S' || self.grid[x][y] == 'a'
    }

    fn is_valid(&self, x: isize, y: isize) -> bool {
        (x >= 0 && x < self.rows as isize) && (y >= 0 && y < self.cols as isize)
    }

    fn bfs(&self) -> Vec<Vec<isize>> {
        const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let (mut queue, mut dist) = (VecDeque::new(), vec![vec![-1; self.cols]; self.rows]);
        let (x, y) = self.find_end();

        dist[x][y] = 0;
        queue.push_back((x, y));
        while !queue.is_empty() {
            let (x1, y1) = queue.pop_front().unwrap();
            for (dx, dy) in DELTAS {
                let (x2, y2) = (x1 as isize + dx, y1 as isize + dy);
                if !self.is_valid(x2, y2) {
                    continue;
                }

                let (x2, y2) = (x2 as usize, y2 as usize);
                if dist[x2][y2] == -1 && self.can_move(x1, y1, x2, y2) {
                    dist[x2][y2] = dist[x1][y1] + 1;
                    queue.push_back((x2, y2))
                }
            }
        }

        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(12);
        assert_eq!("31", Day12.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(12);
        assert_eq!("29", Day12.part_b(&input))
    }
}
