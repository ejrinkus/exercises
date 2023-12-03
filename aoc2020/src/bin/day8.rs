use aoc_helpers::*;
use std::collections::HashSet;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2020
    }
    fn day(&self) -> u32 {
        8
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn execute_instructions(instructions: &Vec<String>) -> (i64, bool) {
    let mut accumulator = 0_i64;
    let mut i = 0_usize;
    let mut indices: HashSet<usize> = HashSet::new();
    loop {
        let line = &instructions[i];
        indices.insert(i);
        let mut pieces = line.split(' ');
        let instruction = pieces.next().unwrap();
        let value = pieces.next().unwrap().parse::<i64>().unwrap();
        match instruction {
            "acc" => {
                accumulator += value;
                i += 1;
            }
            "jmp" => {
                if value >= 0 {
                    i += value as usize;
                } else {
                    i -= (value * -1) as usize;
                }
            }
            _ => i += 1,
        }
        if indices.contains(&i) {
            return (accumulator, true);
        } else if i >= instructions.len() {
            return (accumulator, false);
        }
    }
}

pub fn part_one(input: &str) -> i64 {
    execute_instructions(&input.lines().map(|s| s.to_string()).collect()).0
}

pub fn part_two(input: &str) -> i64 {
    let mut instructions: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    for (i, line) in instructions.clone().iter().enumerate() {
        if line.starts_with("jmp") {
            instructions[i] = line.replace("jmp", "nop");
        } else if line.starts_with("nop") {
            instructions[i] = line.replace("nop", "jmp");
        } else {
            continue;
        }
        let result = execute_instructions(&instructions);
        if !result.1 {
            return result.0;
        } else {
            instructions[i] = line.to_string();
        }
    }
    println!("{}", input);
    0
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            5
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            8
        );
    }
}
