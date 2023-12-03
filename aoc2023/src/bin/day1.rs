use aoc_helpers::*;

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
            let digit = c.to_digit(10);
            if digit.is_some() {
                if first.is_none() {
                    first = Some(digit.expect("uh-oh, tried to unwrap a char that isn't a digit"));
                } else {
                    second = Some(digit.expect("uh-oh, tried to unwrap a char that isn't a digit"));
                }
            } else if inc_text {
                if i + 3 <= l.len() && l.get(i..i + 3) == Some("one") {
                    if first.is_none() {
                        first = Some(1);
                    } else {
                        second = Some(1);
                    }
                } else if i + 3 <= l.len() && l.get(i..i + 3) == Some("two") {
                    if first.is_none() {
                        first = Some(2);
                    } else {
                        second = Some(2);
                    }
                } else if i + 5 <= l.len() && l.get(i..i + 5) == Some("three") {
                    if first.is_none() {
                        first = Some(3);
                    } else {
                        second = Some(3);
                    }
                } else if i + 4 <= l.len() && l.get(i..i + 4) == Some("four") {
                    if first.is_none() {
                        first = Some(4);
                    } else {
                        second = Some(4);
                    }
                } else if i + 4 <= l.len() && l.get(i..i + 4) == Some("five") {
                    if first.is_none() {
                        first = Some(5);
                    } else {
                        second = Some(5);
                    }
                } else if i + 3 <= l.len() && l.get(i..i + 3) == Some("six") {
                    if first.is_none() {
                        first = Some(6);
                    } else {
                        second = Some(6);
                    }
                } else if i + 5 <= l.len() && l.get(i..i + 5) == Some("seven") {
                    if first.is_none() {
                        first = Some(7);
                    } else {
                        second = Some(7);
                    }
                } else if i + 5 <= l.len() && l.get(i..i + 5) == Some("eight") {
                    if first.is_none() {
                        first = Some(8);
                    } else {
                        second = Some(8);
                    }
                } else if i + 4 <= l.len() && l.get(i..i + 4) == Some("nine") {
                    if first.is_none() {
                        first = Some(9);
                    } else {
                        second = Some(9);
                    }
                } else if i + 4 <= l.len() && l.get(i..i + 4) == Some("zero") {
                    if first.is_none() {
                        first = Some(0);
                    } else {
                        second = Some(0);
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
