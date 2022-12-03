// Link - https://adventofcode.com/2022/day/XX
use crate::Solver;

pub struct DayXX;

#[allow(unused_variables)]
impl Solver for DayXX {
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
        let input = read_input(4);
        assert_eq!("", DayXX.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(4);
        assert_eq!("", DayXX.part_b(&input))
    }
}
