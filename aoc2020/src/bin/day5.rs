use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

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
        5
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

// Old solution...
pub fn get_seat(input: &str) -> (u32, u32, u32) {
    let mut row = 0;
    let mut col = 0;
    for (i, c) in input.chars().enumerate() {
        if i < 7 && c == 'F' {
            row -= 2_u32.pow(6 - i as u32);
        } else if i >= 7 && c == 'L' {
            col -= 2_u32.pow(9 - i as u32);
        }
    }
    (row, col, row * 8 + col)
}

// Old solution...
pub fn part_one_old(input: &str) -> u32 {
    let mut highest_id = 0;
    for line in input.lines() {
        let id = get_seat_id(line);
        if id > highest_id {
            highest_id = id;
        }
    }
    highest_id
}

// Old solution...
pub fn part_two_old(input: &str) -> u32 {
    let mut ids: Vec<u32> = Vec::new();
    for line in input.lines() {
        let id = get_seat_id(line);
        ids.push(id);
    }
    ids.sort();
    let mut result = 0;
    for i in 1..ids.len() - 1 {
        if ids[i] + 1 != ids[i + 1] {
            result = ids[i] + 1;
        }
    }
    result
}

// ...new solution
pub fn get_seat_id(input: &str) -> u32 {
    input.chars().enumerate().fold(0, |acc, (i, c)| {
        if c == 'B' || c == 'R' {
            acc | (1 << (9 - i))
        } else {
            acc
        }
    })
}

// ...new solution
pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.chars().enumerate().fold(0, |acc, (i, c)| {
                if c == 'B' || c == 'R' {
                    acc | (1 << (9 - i))
                } else {
                    acc
                }
            })
        })
        .max()
        .unwrap()
}

// ...new solution
pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.chars().enumerate().fold(0, |acc, (i, c)| {
                if c == 'B' || c == 'R' {
                    acc | (1 << (9 - i))
                } else {
                    acc
                }
            })
        })
        .scan((u32::MAX, 0, 0), |state, id| {
            (*state).2 += id;
            if id < (*state).0 {
                (*state).0 = id;
            }
            if id > (*state).1 {
                (*state).1 = id;
            }
            Some(((state.1 * (state.1 + 1)) / 2) - (((state.0 - 1) * (state.0)) / 2) - state.2)
        })
        .last()
        .unwrap()
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(get_seat("BFFFBBFRRR"), (70, 7, 567));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two(""), 0);
    }
}
