use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        6
    }

    fn part_one(&self, _input: &str) -> String {
        "".to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(solution.part_one(""), "");
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(solution.part_two(""), "");
    }
}
