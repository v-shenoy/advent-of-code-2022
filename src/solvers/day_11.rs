// Link - https://adventofcode.com/2022/day/11
use crate::Solver;

pub struct Day11;

#[allow(unused_variables)]
impl Solver for Day11 {
    fn part_a(&self, input: &str) -> String {
        unimplemented!()
    }

    fn part_b(&self, input: &str) -> String {
        unimplemented!()
    }
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
        assert_eq!("", Day11.part_b(&input))
    }
}
