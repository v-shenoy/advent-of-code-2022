// Link - https://adventofcode.com/2022/day/6
use crate::Solver;

pub struct Day06;

impl Solver for Day06 {
    fn part_a(&self, input: &str) -> String {
        const NUM_UNIQUE_CHARS: usize = 4;

        let ans = input
            .as_bytes()
            .windows(NUM_UNIQUE_CHARS)
            // Used a HashSet initially as follows. It gives the right answer but is not as efficient.
            // .position(|chars| chars.iter().collect::<HashSet<_>>().len() == NUM_UNIQUE_CHARS)
            // Then I found this - https://stackoverflow.com/a/46766782/653173, thanks to u/asaaki on reddit.
            .position(|chars| (1..chars.len()).all(|i| !chars[0..i].contains(&chars[i])))
            .unwrap()
            + NUM_UNIQUE_CHARS;

        ans.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        const NUM_UNIQUE_CHARS: usize = 14;

        let ans = input
            .as_bytes()
            .windows(NUM_UNIQUE_CHARS)
            .position(|chars| (1..chars.len()).all(|i| !chars[0..i].contains(&chars[i])))
            .unwrap()
            + NUM_UNIQUE_CHARS;

        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_part_a() {
        let input = read_input(6);
        assert_eq!("7", Day06.part_a(&input))
    }

    #[test]
    fn test_part_b() {
        let input = read_input(6);
        assert_eq!("19", Day06.part_b(&input))
    }
}