// Link - https://adventofcode.com/2022/day/5
use crate::Solver;

pub struct Day05;

impl Solver for Day05 {
    fn part_a(&self, input: &str) -> String {
        let (stack_str, procedure) = input.split_once("\n\n").unwrap();
        let mut stacks = parse_stacks(stack_str);
        let procedure = procedure.lines().map(parse_step);

        procedure.for_each(|(quantity, from, to)| {
            let remaining = stacks[from].len() - quantity;

            let mut popped = stacks[from].split_off(remaining);
            popped.reverse();

            stacks[to].append(&mut popped);
        });

        stacks.iter().filter_map(|s| s.last()).collect()
    }

    fn part_b(&self, input: &str) -> String {
        let (stack_str, procedure) = input.split_once("\n\n").unwrap();
        let mut stacks = parse_stacks(stack_str);
        let procedure = procedure.lines().map(parse_step);

        procedure.for_each(|(quantity, from, to)| {
            let remaining = stacks[from].len() - quantity;
            let mut popped = stacks[from].split_off(remaining);

            stacks[to].append(&mut popped);
        });

        stacks.iter().filter_map(|s| s.last()).collect()
    }
}

fn parse_stacks(stack_str: &str) -> Vec<Vec<char>> {
    let mut stack_str: Vec<_> = stack_str.lines().collect();
    let num_stacks = stack_str.pop().unwrap().split_ascii_whitespace().count();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    stack_str
        .iter()
        .rev()
        .flat_map(|row| {
            row.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_whitespace())
        })
        .for_each(|(i, c)| stacks[i].push(c));

    stacks
}

fn parse_step(step: &str) -> (usize, usize, usize) {
    let mut step = step.split_ascii_whitespace().skip(1).step_by(2);
    let quantity = step.next().unwrap().parse::<usize>().unwrap();
    let from = step.next().unwrap().parse::<usize>().unwrap() - 1;
    let to = step.next().unwrap().parse::<usize>().unwrap() - 1;

    (quantity, from, to)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(5);
        assert_eq!("CMZ", Day05.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(5);
        assert_eq!("MCD", Day05.part_b(&input))
    }
}
