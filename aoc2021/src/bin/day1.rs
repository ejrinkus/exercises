use aoc_helpers::*;

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
        1
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> i64 {
    solution(input, 1)
}

pub fn part_two(input: &str) -> i64 {
    solution(input, 3)
}

fn solution(input: &str, skips: usize) -> i64 {
    let mut count = 0i64;
    input
        .lines()
        .zip(input.lines().skip(skips))
        .for_each(|(x, y)| {
            if x.parse::<i64>().unwrap() < y.parse::<i64>().unwrap() {
                count += 1;
            }
        });
    count
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "199
200
208
210
200
207
240
269
260
263"
            ),
            7
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "199
200
208
210
200
207
240
269
260
263"
            ),
            5
        );
    }
}
