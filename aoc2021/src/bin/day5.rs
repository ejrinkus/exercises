use aoc_helpers::*;
use regex::Regex;
use std::collections::HashMap;

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
        5
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

type MaybeRevIter<T> = Box<dyn DoubleEndedIterator<Item = T>>;

fn parse_line(line: &str, re: &Regex) -> (i64, i64, i64, i64) {
    let caps = re.captures(line).unwrap();
    (
        caps[1].parse::<i64>().unwrap(),
        caps[2].parse::<i64>().unwrap(),
        caps[3].parse::<i64>().unwrap(),
        caps[4].parse::<i64>().unwrap(),
    )
}

fn get_iter(start: i64, end: i64) -> MaybeRevIter<i64> {
    if start <= end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    }
}

// Returns true if this point created a new collision (i.e. the point already
// existed in the map with a quantity of 1).
fn add_point_to_map(point: (i64, i64), map: &mut HashMap<(i64, i64), i64>) -> bool {
    let entry = map.entry(point).or_insert(0);
    *entry += 1;
    *entry == 2
}

fn solution(input: &str, include_diagonals: bool) -> i64 {
    let re: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut overlaps: i64 = 0;
    for line in input.lines() {
        let (x1, y1, x2, y2) = parse_line(line, &re);
        if x1 == x2 {
            // Horizontal
            for y in get_iter(y1, y2) {
                if add_point_to_map((x1, y), &mut map) {
                    overlaps += 1;
                }
            }
        } else if y1 == y2 {
            // Vertical
            for x in get_iter(x1, x2) {
                if add_point_to_map((x, y1), &mut map) {
                    overlaps += 1;
                }
            }
        } else if include_diagonals {
            // Diagonal
            let mut xrange = get_iter(x1, x2);
            let mut yrange = get_iter(y1, y2);
            loop {
                let maybex = xrange.next();
                let maybey = yrange.next();
                if maybex.is_none() || maybey.is_none() {
                    break;
                }
                if add_point_to_map((maybex.unwrap(), maybey.unwrap()), &mut map) {
                    overlaps += 1;
                }
            }
        } else {
            continue;
        }
    }
    overlaps
}

pub fn part_one(input: &str) -> i64 {
    solution(input, false)
}

pub fn part_two(input: &str) -> i64 {
    solution(input, true)
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            ),
            5
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            ),
            12
        );
    }
}
