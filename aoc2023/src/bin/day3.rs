use aoc_helpers::*;
use std::collections::{HashMap, HashSet};

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
        3
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

enum SymbolResponse {
    None,
    Symbol,
    Gear,
}

#[derive(PartialEq)]
enum AdjacencyResponse {
    None,
    Symbol,
    Gears(Vec<(usize, usize)>),
}

pub fn part_one(input: &str) -> u32 {
    let matrix = parse_matrix(input);

    let mut part_valid = false;
    let mut part_start = false;
    let mut part = 0u32;
    let mut sum = 0u32;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let c: char = matrix[row][col];
            let is_digit = c.is_digit(10);
            if !part_start && is_digit {
                // Start of a part.  Part value is initially the value of this digit.
                part = c.to_digit(10).expect("should've been a digit");
                part_start = true;
                part_valid = if part_valid {
                    true
                } else {
                    adjacent_to_symbol(row, col, &matrix) != AdjacencyResponse::None
                };
            } else if is_digit {
                // In the middle of the part.  Multiply current part value by 10 and add the new digit.
                let digit = c.to_digit(10).expect("should've been a digit");
                part = (part * 10) + digit;
                part_valid = if part_valid {
                    true
                } else {
                    adjacent_to_symbol(row, col, &matrix) != AdjacencyResponse::None
                };
            } else if part_start {
                // Just came out of a part.  If it was valid, add it to our sum. Then reset our state.
                if part_valid {
                    sum += part;
                    part_valid = false;
                }
                part = 0;
                part_start = false;
            }
        }
        // After finishing a row, make sure to do the "just came out of a part" logic in case a number
        // was at the end of the row.
        if part_start {
            // Just came out of a part.  If it was valid, add it to our sum. Then reset our state.
            if part_valid {
                sum += part;
                part_valid = false;
            }
            part = 0;
            part_start = false;
        }
    }

    sum
}

pub fn part_two(input: &str) -> u32 {
    let matrix = parse_matrix(input);
    // Each kv pair is (r_gear, c_gear) -> parts where:
    // - r_gear is the row of the gear
    // - c_gear is the column of the gear
    // - parts is all the parts connected to this gear.
    // Note that the only valid gears are those that have exactly 2 parts in their list.
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut part_gears: HashSet<(usize, usize)> = HashSet::new();
    let mut part_start = false;
    let mut part = 0u32;
    let mut sum = 0u32;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let c: char = matrix[row][col];
            let is_digit = c.is_digit(10);
            if !part_start && is_digit {
                // Start of a part.  Part value is initially the value of this digit.
                part = c.to_digit(10).expect("should've been a digit");
                part_start = true;
                match adjacent_to_symbol(row, col, &matrix) {
                    AdjacencyResponse::Gears(g) => {
                        part_gears.extend(g.into_iter());
                    }
                    _ => (),
                }
            } else if is_digit {
                // In the middle of the part.  Multiply current part value by 10 and add the new digit.
                let digit = c.to_digit(10).expect("should've been a digit");
                part = (part * 10) + digit;
                match adjacent_to_symbol(row, col, &matrix) {
                    AdjacencyResponse::Gears(g) => {
                        part_gears.extend(g.into_iter());
                    }
                    _ => (),
                }
            } else if part_start {
                // Just came out of a part.
                for g in part_gears {
                    if !gears.contains_key(&g) {
                        gears.insert(g.clone(), Vec::new());
                    }
                    gears.get_mut(&g).expect("key should've existed").push(part);
                }
                part = 0;
                part_start = false;
                part_gears = HashSet::new();
            }
        }
        // After finishing a row, make sure to do the "just came out of a part" logic in case a number
        // was at the end of the row.
        if part_start {
            for g in part_gears {
                if !gears.contains_key(&g) {
                    gears.insert(g.clone(), Vec::new());
                }
                gears.get_mut(&g).expect("key should've existed").push(part);
            }
            part = 0;
            part_start = false;
            part_gears = HashSet::new();
        }
    }

    for parts in gears.values() {
        if parts.len() != 2 {
            continue;
        }
        sum += parts[0] * parts[1];
    }

    sum
}

fn is_symbol(c: char) -> SymbolResponse {
    match c {
        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => SymbolResponse::None,
        '*' => SymbolResponse::Gear,
        _ => SymbolResponse::Symbol,
    }
}

fn adjacent_to_symbol(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> AdjacencyResponse {
    let mut gears: Vec<(usize, usize)> = Vec::new();
    let mut resp: AdjacencyResponse = AdjacencyResponse::None;

    let top = row == 0;
    let bottom = row == matrix.len() - 1;
    let left = col == 0;
    let right = col == matrix[0].len() - 1;
    if !top && !left {
        // top-left
        let val = matrix[row - 1][col - 1];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row - 1, col - 1));
            }
        }
    }
    if !top {
        // top
        let val = matrix[row - 1][col];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row - 1, col));
            }
        }
    }
    if !top && !right {
        // top-right
        let val = matrix[row - 1][col + 1];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row - 1, col + 1));
            }
        }
    }
    if !left {
        // left
        let val = matrix[row][col - 1];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row, col - 1));
            }
        }
    }
    if !right {
        // right
        let val = matrix[row][col + 1];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row, col + 1));
            }
        }
    }
    if !bottom && !left {
        // bottom-left
        let val = matrix[row + 1][col - 1];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row + 1, col - 1));
            }
        }
    }
    if !bottom {
        // bottom
        let val = matrix[row + 1][col];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row + 1, col));
            }
        }
    }
    if !bottom && !right {
        // bottom-right
        let val = matrix[row + 1][col + 1];
        let sym = is_symbol(val);
        match sym {
            SymbolResponse::None => (),
            SymbolResponse::Symbol => {
                resp = AdjacencyResponse::Symbol;
            }
            SymbolResponse::Gear => {
                gears.push((row + 1, col + 1));
            }
        }
    }

    if gears.len() > 0 {
        resp = AdjacencyResponse::Gears(gears);
    }

    resp
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "4361"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "467835"
        );
    }
}
