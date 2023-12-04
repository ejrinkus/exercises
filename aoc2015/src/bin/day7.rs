use aoc_2015_libs::circuit::Circuit;
use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use regex::{Captures, Regex};

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        7
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut c = Circuit::new();
    for line in input.lines() {
        c.handle_instruction(line).unwrap();
    }
    if let Err(err) = c.resolve() {
        panic!("{}", err);
    }
    c.get_wire("a").unwrap() as i64
}

pub fn part_two(input: &str) -> i64 {
    // Same solution, different input.
    let re = Regex::new(r"\d+( -> b)").unwrap();
    let new_input = re.replace(input, |caps: &Captures| {
        format!("{}{}", part_one(input), &caps[1])
    });
    part_one(&new_input)
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let input = "123 -> x
456 -> y
x AND y -> a";
        assert_eq!(part_one(input), 72);
        println!("success");

        let input = "123 -> x
456 -> y
x OR y -> a";
        assert_eq!(part_one(input), 507);
        println!("success");

        let input = "123 -> x
456 -> y
x LSHIFT 2 -> a";
        assert_eq!(part_one(input), 492);
        println!("success");

        let input = "123 -> x
456 -> y
y RSHIFT 2 -> a";
        assert_eq!(part_one(input), 114);
        println!("success");

        let input = "123 -> x
NOT x -> a";
        assert_eq!(part_one(input), 65412);
        println!("success");

        let input = "456 -> y
NOT y -> a";
        assert_eq!(part_one(input), 65079);
        println!("success");
    }
}
