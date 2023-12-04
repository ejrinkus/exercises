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
        solution(input, false).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        solution(input, true).to_string()
    }
}

fn solution(input: &str, inc_text: bool) -> u32 {
    let mut sum = 0u32;
    input.lines().for_each(|l| {
        let mut first: Option<u32> = None;
        let mut second: Option<u32> = None;
        l.chars().enumerate().for_each(|(i, c)| {
            if let Some(digit) = c.to_digit(10) {
                if first.is_none() {
                    first = Some(digit);
                } else {
                    second = Some(digit);
                }
            } else if inc_text {
                if let Some(digit) = text_to_digit(l, i) {
                    if first.is_none() {
                        first = Some(digit.into());
                    } else {
                        second = Some(digit.into());
                    }
                }
            }
        });
        let unwrapped_first = first.unwrap_or(0);
        let unwrapped_second = second.unwrap_or(unwrapped_first);
        sum += (unwrapped_first * 10) + unwrapped_second;
    });
    sum
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
7pqrstsixteen"
            ),
            "281"
        );
    }
}
