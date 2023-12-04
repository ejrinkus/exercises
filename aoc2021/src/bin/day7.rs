use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::cmp::min;

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
        7
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> usize {
    let mut sorted: Vec<usize> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    sorted.sort_unstable();
    let mut l_ptr = 0;
    let mut r_ptr = sorted.len() - 1;
    let mut l_fuel = 0;
    let mut r_fuel = 0;
    while l_ptr < r_ptr {
        l_fuel += (sorted[l_ptr + 1] - sorted[l_ptr]) * (l_ptr + 1);
        l_ptr += 1;
        r_fuel += (sorted[r_ptr] - sorted[r_ptr - 1]) * (sorted.len() - r_ptr);
        r_ptr -= 1;
    }
    l_fuel + r_fuel
}

fn p2_fuel_for_location(crabs: &Vec<usize>, end: usize) -> usize {
    let mut fuel = 0;
    for c in crabs {
        let n = if *c < end { end - c } else { c - end };
        fuel += (n * (n + 1)) / 2;
    }
    fuel
}

pub fn part_two(input: &str) -> usize {
    let crabs: Vec<usize> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let average = (crabs.iter().sum::<usize>() as f64) / (crabs.len() as f64);
    // The correct end-position is either the average rounded up, or rounded down.
    // Try both and use the better answer.
    min(
        p2_fuel_for_location(&crabs, average.floor() as usize),
        p2_fuel_for_location(&crabs, average.ceil() as usize),
    )
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(part_one("16,1,2,0,4,2,7,1,2,14"), 37);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two("16,1,2,0,4,2,7,1,2,14"), 168);
    }
}
