use aoc_helpers::*;

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
    for (i, l1) in input.lines().enumerate() {
        for (_j, l2) in input.lines().enumerate().skip(i + 1) {
            let first = l1.parse::<i64>().unwrap();
            let second = l2.parse::<i64>().unwrap();
            if first + second == 2020 {
                return first * second;
            }
        }
    }
    0
}

pub fn part_two(input: &str) -> i64 {
    for (i, l1) in input.lines().enumerate() {
        for (j, l2) in input.lines().enumerate().skip(i + 1) {
            for (_k, l3) in input.lines().enumerate().skip(j + 1) {
                let first = l1.parse::<i64>().unwrap();
                let second = l2.parse::<i64>().unwrap();
                let third = l3.parse::<i64>().unwrap();
                if first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "1721
979
366
299
675
1456"
            ),
            514579
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "1721
979
366
299
675
1456"
            ),
            241861950
        );
    }
}
