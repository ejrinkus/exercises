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
        1
    }

    fn part_one(&self, input: &str) -> String {
        solution(input, false)
    }

    fn part_two(&self, input: &str) -> String {
        solution(input, true)
    }
}

fn solution(input: &str, inc_text: bool) -> String {
    let mut sum = 0u32;
    for line in input.lines() {
        let mut first_digit: Option<u8> = None;
        let mut second_digit: Option<u8> = None;
        let mut remainder = line;
        while let (rem, Some(num)) = get_next_digit(remainder, inc_text, false) {
            remainder = rem;
            if first_digit.is_none() {
                first_digit = Some(num);
            }
            second_digit = Some(num);
        }
        sum += ((first_digit.unwrap_or(0) as u32) * 10) + (second_digit.unwrap_or(0) as u32);
    }
    sum.to_string()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            "142"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
zoneight"
            ),
            "299"
        );
    }
}
