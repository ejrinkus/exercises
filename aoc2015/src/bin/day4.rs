use aoc_helpers::*;
use md5::*;

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
        4
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> i64 {
    find_secret(input, 5)
}

pub fn part_two(input: &str) -> i64 {
    find_secret(input, 6)
}

pub fn validate(digest: &Digest, zeroes: u8) -> bool {
    if zeroes > 16 {
        return false;
    }
    let limit = zeroes / 2;
    for i in 0..limit {
        if digest[i as usize] != 0 {
            return false;
        }
    }
    if zeroes % 2 == 1 && digest[limit as usize] >= 16 {
        return false;
    }
    return true;
}

pub fn find_secret(input: &str, zeroes: u8) -> i64 {
    if zeroes > 16 {
        return -1;
    }
    let mut i: i64 = 0;
    loop {
        i += 1;
        let key = format!("{}{}", input.trim(), i);
        let digest = md5::compute(key);
        if validate(&digest, zeroes) {
            return i;
        }
    }
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(part_one("abcdef"), 609043);
        assert_eq!(part_one("pqrstuv"), 1048970);
    }
}
