// Link - https://adventofcode.com/2022/day/08
use crate::Solver;

pub struct Day08;

impl Solver for Day08 {
    fn part_a(&self, input: &str) -> String {
        let trees: Vec<Vec<i8>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
            .collect();

        // Slower,  but less space: O(n*m*(n+m)) solution.
        // let rows = trees.len();
        // let cols = trees[0].len();

        // let mut ans: usize = 0;
        // for row in 0..rows {
        //     for col in 0..cols {
        //         let height = trees[row][col];
        //         if (0..row).all(|i| trees[i][col] < height)
        //             || (row + 1..rows).all(|i| trees[i][col] < height)
        //             || (0..col).all(|i| trees[row][i] < height)
        //             || (col + 1..cols).all(|i| trees[row][i] < height)
        //         {
        //             ans += 1;
        //         }
        //     }
        // }

        // Faster, but uses more space: O(n*m) solution. O(n*m) more space
        // required for the 2-d visibility vector.
        let mut visible = vec![vec![false; trees[0].len()]; trees.len()];

        for row in 0..trees.len() {
            row_visibility(trees.as_slice(), visible.as_mut_slice(), row, true);
            row_visibility(trees.as_slice(), visible.as_mut_slice(), row, false);
        }

        for col in 0..trees[0].len() {
            col_visibility(trees.as_slice(), visible.as_mut_slice(), col, true);
            col_visibility(trees.as_slice(), visible.as_mut_slice(), col, false);
        }

        let ans: usize = visible
            .iter()
            .map(|row| row.iter().filter(|v| **v).count())
            .sum();
        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let trees: Vec<Vec<i8>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
            .collect();

        // let rows = trees.len();
        // let cols = trees[0].len();

        // let mut ans: usize = 0;
        // for row in 0..rows {
        //     for col in 0..cols {
        //         let height = trees[row][col];

        //         let top = (0..row).rfind(|&i| trees[i][col] >= height).unwrap_or(0);
        //         let bottom = (row + 1..rows)
        //             .find(|&i| trees[i][col] >= height)
        //             .unwrap_or(rows - 1);
        //         let left = (0..col).rfind(|&i| trees[row][i] >= height).unwrap_or(0);
        //         let right = (col + 1..cols)
        //             .find(|&i| trees[row][i] >= height)
        //             .unwrap_or(cols - 1);

        //         let vd = (row - top) * (bottom - row) * (col - left) * (right - col);
        //         if vd > ans {
        //             ans = vd;
        //         }
        //     }
        // }

        let mut viewing_dist = vec![vec![1; trees[0].len()]; trees.len()];

        for row in 0..trees.len() {
            row_viewing_dist(trees.as_slice(), viewing_dist.as_mut_slice(), row, true);
            row_viewing_dist(trees.as_slice(), viewing_dist.as_mut_slice(), row, false);
        }

        for col in 0..trees[0].len() {
            col_viewing_dist(trees.as_slice(), viewing_dist.as_mut_slice(), col, true);
            col_viewing_dist(trees.as_slice(), viewing_dist.as_mut_slice(), col, false);
        }

        let ans: usize = *viewing_dist.iter().flatten().max().unwrap();
        ans.to_string()
    }
}

fn row_visibility(trees: &[Vec<i8>], visible: &mut [Vec<bool>], row: usize, is_ltr: bool) {
    let mut max: i8 = -1;

    if is_ltr {
        for col in 0..trees[0].len() {
            if trees[row][col] > max {
                visible[row][col] = true;
                max = trees[row][col]
            }
        }
    } else {
        for col in (0..trees[0].len()).rev() {
            if trees[row][col] > max {
                visible[row][col] = true;
                max = trees[row][col]
            }
        }
    }
}

fn col_visibility(trees: &[Vec<i8>], visible: &mut [Vec<bool>], col: usize, is_ttb: bool) {
    let mut max: i8 = -1;

    if is_ttb {
        for row in 0..trees.len() {
            if trees[row][col] > max {
                visible[row][col] = true;
                max = trees[row][col]
            }
        }
    } else {
        for row in (0..trees.len()).rev() {
            if trees[row][col] > max {
                visible[row][col] = true;
                max = trees[row][col]
            }
        }
    }
}

struct Tree {
    height: i8,
    idx: usize,
}

fn row_viewing_dist(trees: &[Vec<i8>], viewing_dist: &mut [Vec<usize>], row: usize, is_ltr: bool) {
    let mut stack: Vec<Tree> = Vec::new();

    if is_ltr {
        for col in (0..trees[0].len()).rev() {
            while let Some(tree) = stack.last() {
                if tree.height >= trees[row][col] {
                    break;
                }

                stack.pop();
            }

            match stack.last() {
                Some(tree) => viewing_dist[row][col] *= tree.idx.saturating_sub(col),
                None => viewing_dist[row][col] *= (trees[0].len() - 1).saturating_sub(col),
            }

            stack.push(Tree {
                height: trees[row][col],
                idx: col,
            });
        }
    } else {
        for col in 0..trees[0].len() {
            while let Some(tree) = stack.last() {
                if tree.height >= trees[row][col] {
                    break;
                }

                stack.pop();
            }

            match stack.last() {
                Some(tree) => viewing_dist[row][col] *= col.saturating_sub(tree.idx),
                None => viewing_dist[row][col] *= (col).saturating_sub(0),
            }

            stack.push(Tree {
                height: trees[row][col],
                idx: col,
            });
        }
    }
}

fn col_viewing_dist(trees: &[Vec<i8>], viewing_dist: &mut [Vec<usize>], col: usize, is_ttb: bool) {
    let mut stack: Vec<Tree> = Vec::new();

    if is_ttb {
        for row in (0..trees.len()).rev() {
            while let Some(tree) = stack.last() {
                if tree.height >= trees[row][col] {
                    break;
                }

                stack.pop();
            }

            match stack.last() {
                Some(tree) => viewing_dist[row][col] *= tree.idx.saturating_sub(row),
                None => viewing_dist[row][col] *= (trees.len() - 1).saturating_sub(row),
            }

            stack.push(Tree {
                height: trees[row][col],
                idx: row,
            });
        }
    } else {
        for row in 0..trees.len() {
            while let Some(tree) = stack.last() {
                if tree.height >= trees[row][col] {
                    break;
                }

                stack.pop();
            }

            match stack.last() {
                Some(tree) => viewing_dist[row][col] *= row.saturating_sub(tree.idx),
                None => viewing_dist[row][col] *= (row).saturating_sub(0),
            }

            stack.push(Tree {
                height: trees[row][col],
                idx: row,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(8);
        assert_eq!("21", Day08.part_a(&input));
    }

    #[test]
    fn test_part_b() {
        let input = read_input(8);
        assert_eq!("8", Day08.part_b(&input));
    }
}
