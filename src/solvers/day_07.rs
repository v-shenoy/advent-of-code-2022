// Link - https://adventofcode.com/2022/day/7
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use crate::Solver;
use Node::*;

pub struct Day07;

impl Solver for Day07 {
    fn part_a(&self, input: &str) -> String {
        let root = parse_file_system(input);

        let limit = 100000;
        root.size_at_most(limit).to_string()
    }

    fn part_b(&self, input: &str) -> String {
        let root = parse_file_system(input);

        let total = 70000000;
        let required = 30000000;
        let curr = total - root.size();
        let to_free = required - curr;

        root.free_up_space(to_free).unwrap().to_string()
    }
}

#[derive(Debug, Clone)]
enum Node<'a> {
    File(u64),
    Dir {
        parent: Option<Rc<RefCell<Node<'a>>>>,
        sub_dirs: HashMap<&'a str, Rc<RefCell<Node<'a>>>>,
    },
}

impl<'a> Node<'a> {
    fn size(&self) -> u64 {
        match self {
            File(size) => *size,
            Dir { sub_dirs, .. } => sub_dirs.values().map(|n| n.borrow().size()).sum(),
        }
    }

    fn size_at_most(&self, limit: u64) -> u64 {
        match self {
            File(_) => 0,
            Dir { sub_dirs, .. } => {
                let size = self.size();
                let ans = if size <= limit { size } else { 0 }
                    + sub_dirs
                        .values()
                        .map(|n| n.borrow().size_at_most(limit))
                        .sum::<u64>();

                ans
            }
        }
    }

    fn free_up_space(&self, to_free: u64) -> Option<u64> {
        match self {
            File(_) => None,
            Dir { sub_dirs, .. } => {
                let sub_dir_size = sub_dirs
                    .values()
                    .filter_map(|n| n.borrow().free_up_space(to_free))
                    .min();

                if sub_dir_size.is_some() {
                    return sub_dir_size;
                }

                let size = self.size();
                if size >= to_free {
                    return Some(size);
                }

                None
            }
        }
    }
}

fn parse_file_system(input: &str) -> Node {
    let root = Rc::new(RefCell::new(Dir {
        parent: None,
        sub_dirs: Default::default(),
    }));

    let mut curr = Rc::clone(&root);
    for line in input.trim().lines() {
        let l = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let l = l.as_slice();

        match l {
            ["$", "cd", dest] => match *dest {
                "/" => curr = Rc::clone(&root),
                ".." => {
                    let Dir {ref parent, .. } = curr.borrow().clone() else { panic!() };
                    curr = Rc::clone(parent.as_ref().unwrap());
                }
                dest => {
                    let Dir {ref sub_dirs, .. } = curr.borrow().clone() else { panic!() };
                    curr = Rc::clone(sub_dirs.get(dest).unwrap());
                }
            },
            ["$", "ls"] => (),
            ["dir", sub_dir] => {
                let node = Dir {
                    parent: Some(Rc::clone(&curr)),
                    sub_dirs: Default::default(),
                };

                let Dir {ref mut sub_dirs, .. } = *curr.borrow_mut() else { panic!() };
                sub_dirs.insert(sub_dir, Rc::new(RefCell::new(node)));
            }
            [size, file] => {
                let node = File(size.parse().unwrap());

                let Dir {ref mut sub_dirs, .. } = *curr.borrow_mut() else { panic!() };
                sub_dirs.insert(file, Rc::new(RefCell::new(node)));
            }
            _ => unreachable!(),
        }
    }

    return root.borrow().clone();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(7);
        assert_eq!("95437", Day07.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(7);
        assert_eq!("24933642", Day07.part_b(&input))
    }
}
