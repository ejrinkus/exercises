use aoc_helpers::*;
use std::collections::HashMap;
use std::time::Instant;

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
        15
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> u64 {
    let mut nums: Vec<u64> = input
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut last_num = nums.pop().unwrap();
    let mut num_turns: HashMap<u64, u64> = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        num_turns.insert(*n, i as u64 + 1);
    }
    for i in (nums.len() + 1)..2020 {
        if num_turns.contains_key(&last_num) {
            let turn = *num_turns.get(&last_num).unwrap();
            num_turns.insert(last_num, i as u64);
            last_num = i as u64 - turn;
        } else {
            num_turns.insert(last_num, i as u64);
            last_num = 0;
        }
    }
    last_num
}

pub fn part_two(input: &str) -> u64 {
    let mut nums: Vec<u64> = input
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut last_num = nums.pop().unwrap();
    let mut num_turns: HashMap<u64, u64> = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        num_turns.insert(*n, i as u64 + 1);
    }
    let start = Instant::now();
    for i in (nums.len() + 1)..30000000 {
        // Using the entry API, the whole thing takes about ~18 seconds.  The
        // original solution that's still being used in part 1, however, takes
        // ~44 seconds if used for this part.
        // EDIT: 1.8s for the below with cargo run ... --release
        num_turns
            .entry(last_num)
            .and_modify(|e| {
                last_num = i as u64 - *e;
                *e = i as u64
            })
            .or_insert_with(|| {
                last_num = 0;
                i as u64
            });
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    last_num
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(part_one("0,3,6"), 436);
        assert_eq!(part_one("1,3,2"), 1);
        assert_eq!(part_one("2,1,3"), 10);
        assert_eq!(part_one("1,2,3"), 27);
        assert_eq!(part_one("2,3,1"), 78);
        assert_eq!(part_one("3,2,1"), 438);
        assert_eq!(part_one("3,1,2"), 1836);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two("0,3,6"), 175594);
    }
}
