use aoc_helpers::*;
use std::collections::HashMap;
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
        6
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut set: HashSet<char> = HashSet::new();
    let mut count = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            count += set.len();
            set.clear();
        }
        for c in line.trim().chars() {
            set.insert(c);
        }
    }
    count += set.len();
    count as i64
}

pub fn part_two(input: &str) -> i64 {
    let mut set: HashMap<char, i64> = HashMap::new();
    let mut count = 0;
    let mut group_size = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            for (_k, v) in set.clone() {
                if v == group_size {
                    count += 1;
                }
            }
            group_size = 0;
            set.clear();
            continue;
        }
        for c in line.trim().chars() {
            let e = set.entry(c).or_insert(0);
            *e += 1;
        }
        group_size += 1;
    }
    for (_k, v) in set {
        if v == group_size {
            count += 1;
        }
    }
    count as i64
}

#[cfg(test)]
mod day6_tests {
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
