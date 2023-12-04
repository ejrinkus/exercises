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
        6
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

// For a lanternfish starting with value |start|, return the number of fish that
// it will produce (plus itself) after |days| days.
fn simulate(start: u64, days: u64) -> u64 {
    let mut fishes: Vec<u64> = Vec::new();
    fishes.push(start);
    for _d in 0..days {
        let mut new_fishes: u64 = 0;
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                new_fishes += 1;
                fishes[i] = 6;
            } else {
                fishes[i] -= 1;
            }
        }
        for _i in 0..new_fishes {
            fishes.push(8);
        }
    }
    fishes.len() as u64
}

// For a lanternfish starting with value |start|, return the number of fish that
// it will produce (plus itself) after |days| days.
fn simulate256(start: u64) -> u64 {
    // Memoize the first 128 days for each fish value.
    let memo: [u64; 9] = [
        simulate(0, 128),
        simulate(1, 128),
        simulate(2, 128),
        simulate(3, 128),
        simulate(4, 128),
        simulate(5, 128),
        simulate(6, 128),
        simulate(7, 128),
        simulate(8, 128),
    ];

    // Simulate a single fish out 128 days.
    let mut fishes: Vec<u64> = Vec::new();
    fishes.push(start);
    for _d in 0..128 {
        let mut new_fishes: u64 = 0;
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                new_fishes += 1;
                fishes[i] = 6;
            } else {
                fishes[i] -= 1;
            }
        }
        for _i in 0..new_fishes {
            fishes.push(8);
        }
    }

    // Use the memoized values to get the final count for days 129-256.
    let mut sum: u64 = 0;
    for fish in fishes {
        sum += memo[fish as usize];
    }
    sum
}

pub fn part_one(input: &str) -> u64 {
    let memo: [u64; 7] = [
        simulate(0, 80),
        simulate(1, 80),
        simulate(2, 80),
        simulate(3, 80),
        simulate(4, 80),
        simulate(5, 80),
        simulate(6, 80),
    ];
    let mut sum: u64 = 0;
    for fish in input.trim().split(",") {
        sum += memo[fish.parse::<u64>().unwrap() as usize];
    }
    sum
}

pub fn part_two(input: &str) -> u64 {
    let memo: [u64; 7] = [
        simulate256(0),
        simulate256(1),
        simulate256(2),
        simulate256(3),
        simulate256(4),
        simulate256(5),
        simulate256(6),
    ];
    let mut sum: u64 = 0;
    for fish in input.trim().split(",") {
        sum += memo[fish.parse::<u64>().unwrap() as usize];
    }
    sum
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(part_one("3,4,3,1,2"), 5934);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two("3,4,3,1,2"), 26984457539);
    }
}
