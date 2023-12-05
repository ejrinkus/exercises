use aoc_helpers::runner::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::error::Error;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::str::Lines;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        5
    }

    fn part_one(&self, input: &str) -> String {
        let mut line_iter = input.lines();
        let seed_line = line_iter.next().expect("Didn't get a seed line");
        let mut seeds = get_seeds(seed_line);

        line_iter.next(); // Throw away empty line

        for _i in 0..7 {
            line_iter.next(); // Throw away map line
            seeds = update_seed_values(&mut line_iter, seeds);
        }

        seeds.into_iter().min().unwrap().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut line_iter = input.lines();
        let seed_line = line_iter.next().expect("Didn't get a seed line");
        let mut ranges = get_seeds_as_ranges(seed_line);

        line_iter.next(); // Throw away empty line

        for _i in 0..7 {
            line_iter.next(); // Throw away map line
            ranges = update_ranges(&mut line_iter, ranges);
        }

        ranges
            .into_iter()
            .min_by(|r1, r2| r1.0.cmp(&r2.0))
            .unwrap()
            .0
            .to_string()
    }
}

fn get_seeds(line: &str) -> VecDeque<u64> {
    let (mut remainder, _): (&str, &str) =
        tag::<&str, &str, Error<&str>>("seeds: ")(line).expect("Failed to parse seed line");

    let mut seeds: VecDeque<u64> = VecDeque::new();
    while let Ok((rem, num)) = u64::<&str, Error<&str>>(remainder) {
        seeds.push_back(num);
        if let Ok((trimmed, _)) = space1::<&str, Error<&str>>(rem) {
            remainder = trimmed;
        } else {
            remainder = rem;
        }
    }
    seeds
}

fn get_seeds_as_ranges(line: &str) -> VecDeque<(u64, u64)> {
    let (mut remainder, _): (&str, &str) =
        tag::<&str, &str, Error<&str>>("seeds: ")(line).expect("Failed to parse seed line");

    let mut seeds: VecDeque<u64> = VecDeque::new();
    while let Ok((rem, num)) = u64::<&str, Error<&str>>(remainder) {
        seeds.push_back(num);
        if let Ok((trimmed, _)) = space1::<&str, Error<&str>>(rem) {
            remainder = trimmed;
        } else {
            remainder = rem;
        }
    }
    let mut ranges: VecDeque<(u64, u64)> = VecDeque::with_capacity(seeds.len() / 2);
    for _i in 0..(seeds.len() / 2) {
        let start = seeds.pop_front().unwrap();
        let length = seeds.pop_front().unwrap();
        ranges.push_back((start, length));
    }
    ranges
}

fn update_seed_values(lines: &mut Lines, mut old_vals: VecDeque<u64>) -> VecDeque<u64> {
    let mut new_vals: VecDeque<u64> = VecDeque::with_capacity(old_vals.len());
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let (dest_start, src_start, length) = parse_map_line(line);
        for _i in 0..old_vals.len() {
            let v = old_vals.pop_front().unwrap();
            if v < src_start {
                old_vals.push_back(v);
                continue;
            }
            let diff = v - src_start;
            if diff < length {
                new_vals.push_back(dest_start + diff);
            } else {
                old_vals.push_back(v);
            }
        }
    }
    for v in old_vals.into_iter() {
        new_vals.push_back(v);
    }
    new_vals
}

fn update_ranges(lines: &mut Lines, mut old_ranges: VecDeque<(u64, u64)>) -> VecDeque<(u64, u64)> {
    let mut new_ranges: VecDeque<(u64, u64)> = VecDeque::with_capacity(old_ranges.len());
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let (dest_start, src_start, length) = parse_map_line(line);
        for _i in 0..old_ranges.len() {
            let range = old_ranges.pop_front().unwrap();
            if range.0 + range.1 < src_start || src_start + length < range.0 {
                old_ranges.push_back(range);
                continue;
            }

            let start_max = max(range.0, src_start);
            let new_length = min(src_start + length, range.0 + range.1) - start_max;
            let new_start = ((start_max as i64) - (src_start as i64) + (dest_start as i64)) as u64;
            let new_range = (new_start, new_length);

            if range.0 < start_max {
                old_ranges.push_back((range.0, start_max - range.0));
            }
            if start_max + new_length < range.0 + range.1 {
                old_ranges.push_back((
                    start_max + new_length,
                    (range.0 + range.1) - (start_max + new_length),
                ));
            }
            new_ranges.push_back(new_range);
        }
    }
    for r in old_ranges.into_iter() {
        new_ranges.push_back(r);
    }

    new_ranges
}

fn parse_map_line(line: &str) -> (u64, u64, u64) {
    let mut read_result = u64::<&str, Error<&str>>(line).unwrap();
    let mut rem = read_result.0;
    let dest_start = read_result.1;
    let mut trim_result = space1::<&str, Error<&str>>(rem).unwrap();
    let mut remainder = trim_result.0;

    read_result = u64::<&str, Error<&str>>(remainder).unwrap();
    rem = read_result.0;
    let src_start = read_result.1;
    trim_result = space1::<&str, Error<&str>>(rem).unwrap();
    remainder = trim_result.0;

    read_result = u64::<&str, Error<&str>>(remainder).unwrap();
    let length = read_result.1;

    (dest_start, src_start, length)
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            "35"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            "46"
        );
    }
}
