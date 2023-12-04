use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        21
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(_input: &str) -> i64 {
    0
}

pub fn part_two(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(part_one(""), 0);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two(""), 0);
    }
}
