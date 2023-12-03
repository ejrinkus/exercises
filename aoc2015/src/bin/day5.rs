use aoc_helpers::*;
use std::collections::HashSet;

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
        5
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut count: i64 = 0;
    for line in input.lines() {
        if is_nice_part1(line) {
            count += 1;
        }
    }
    count
}

pub fn part_two(input: &str) -> i64 {
    let mut count: i64 = 0;
    for line in input.lines() {
        if is_nice_part2(line) {
            count += 1;
        }
    }
    count
}

pub fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

pub fn is_naughty_pair(c1: char, c2: char) -> bool {
    (c1 == 'a' && c2 == 'b')
        || (c1 == 'c' && c2 == 'd')
        || (c1 == 'p' && c2 == 'q')
        || (c1 == 'x' && c2 == 'y')
}

pub fn is_nice_part1(input: &str) -> bool {
    if input.len() < 3 {
        // Need at least 3 vowels.
        return false;
    }
    let mut vowels: u8 = 0;
    let mut found_double = false;
    // 2 char sliding window
    let mut chars = input.chars();
    let mut c1 = chars.next().unwrap();
    let mut c2 = chars.next().unwrap();
    if is_vowel(c1) {
        vowels = 1;
    }
    loop {
        if is_naughty_pair(c1, c2) {
            return false;
        }
        if is_vowel(c2) {
            vowels += 1;
        }
        if c1 == c2 {
            found_double = true;
        }
        c1 = c2;
        c2 = match chars.next() {
            Some(c) => c,
            None => break,
        }
    }
    return vowels >= 3 && found_double;
}

pub fn is_nice_part2(input: &str) -> bool {
    if input.len() < 4 {
        // Need pair that repeats at least twice.
        return false;
    }
    let mut pairs: HashSet<(char, char)> = HashSet::new();
    let mut found_double = false;
    let mut found_repeat = false;
    // 3 char sliding window
    let mut chars = input.chars();
    let mut c1 = chars.next().unwrap();
    let mut c2 = chars.next().unwrap();
    let mut c3 = chars.next().unwrap();
    let mut checkpair = true;
    loop {
        if checkpair && !found_repeat {
            let pair = (c1, c2);
            if pairs.contains(&pair) {
                found_repeat = true;
            } else {
                pairs.insert(pair);
            }
            if c1 == c2 && c1 == c3 {
                // we can't fulfill the 'repeated pair' condition with
                // overlapping pairs.  This can only happen when the
                // same char is repeated 3 times in a row.  So in these
                // situations, we add the pair once, and then skip this
                // step on the second iteration so we don't accidentally
                // accept the overlap.
                checkpair = false;
            }
        } else {
            checkpair = true;
        }
        if c1 == c3 {
            found_double = true;
        }
        if c3 == 0 as char {
            break;
        }
        c1 = c2;
        c2 = c3;
        c3 = match chars.next() {
            Some(c) => c,
            None => 0 as char,
        };
    }
    return found_repeat && found_double;
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert!(is_nice_part1("ugknbfddgicrmopn"));
        assert!(is_nice_part1("aaa"));
        assert!(!is_nice_part1("jchzalrnumimnmhp"));
        assert!(!is_nice_part1("haegwjzuvuyypxyu"));
        assert!(!is_nice_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn samples_part2() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }
}
