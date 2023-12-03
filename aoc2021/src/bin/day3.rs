use aoc_helpers::*;
use std::vec::Vec;

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
        3
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

const NUMS_LENGTH: usize = 12;

pub fn part_one(input: &str) -> i64 {
    let mut gamma: [i32; NUMS_LENGTH] = [0; NUMS_LENGTH];
    let mut epsilon: [i32; NUMS_LENGTH] = [1; NUMS_LENGTH];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                gamma[i] += 1;
            } else {
                gamma[i] -= 1;
            }
        }
    }
    for i in 0..NUMS_LENGTH {
        if gamma[i] > 0 {
            gamma[i] = 1;
            epsilon[i] = 0;
        } else {
            gamma[i] = 0;
            epsilon[i] = 1;
        }
    }
    let gamma_num = gamma.iter().fold(0, |acc, x| (acc * 2) + x);
    let epsilon_num = epsilon.iter().fold(0, |acc, x| (acc * 2) + x);
    (gamma_num * epsilon_num).into()
}

pub fn part_two(input: &str) -> i64 {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut oxygen: i64 = 0;
    for i in 0..NUMS_LENGTH {
        let (zeroes, ones) = partition(lines, i);
        if zeroes.len() > ones.len() {
            lines = zeroes;
        } else {
            lines = ones;
        }
        if lines.len() == 1 {
            oxygen = lines
                .pop()
                .unwrap()
                .as_bytes()
                .iter()
                .fold(0i64, |acc, x| (acc * 2) + (*x as i64 - 48i64));
            break;
        }
    }
    lines = input.lines().collect();
    let mut c02: i64 = 0;
    for i in 0..NUMS_LENGTH {
        let (zeroes, ones) = partition(lines, i);
        if zeroes.len() <= ones.len() {
            lines = zeroes;
        } else {
            lines = ones;
        }
        if lines.len() == 1 {
            c02 = lines
                .pop()
                .unwrap()
                .as_bytes()
                .iter()
                .fold(0i64, |acc, x| (acc * 2) + (*x as i64 - 48i64));
            break;
        }
    }
    (oxygen * c02).into()
}

pub fn partition(mut numbers: Vec<&str>, i: usize) -> (Vec<&str>, Vec<&str>) {
    let mut zeroes: Vec<&str> = Vec::new();
    let mut ones: Vec<&str> = Vec::new();
    while !numbers.is_empty() {
        let num = numbers.pop().unwrap();
        if num.as_bytes()[i] == 48 {
            zeroes.push(num);
        } else {
            ones.push(num);
        }
    }
    (zeroes, ones)
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            ),
            198
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            ),
            230
        );
    }
}
