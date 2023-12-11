use std::collections::HashSet;

use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

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
        11
    }

    fn part_one(&self, input: &str) -> String {
        solve(input, 2)
    }

    fn part_two(&self, input: &str) -> String {
        solve(input, 1000000)
    }
}

fn solve(input: &str, expansion: u128) -> String {
    let space = parse_matrix(input);

    let mut empty_rows: HashSet<usize> = HashSet::with_capacity(space.len());
    for row in 0..space.len() {
        if is_row_empty(&space, row) {
            empty_rows.insert(row);
        }
    }

    let mut empty_cols: HashSet<usize> = HashSet::with_capacity(space[0].len());
    for col in 0..space[0].len() {
        if is_col_empty(&space, col) {
            empty_cols.insert(col);
        }
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for row in 0..space.len() {
        for col in 0..space[0].len() {
            if space[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut sum = 0u128;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let max_row = std::cmp::max(galaxies[i].0, galaxies[j].0);
            let min_row = std::cmp::min(galaxies[i].0, galaxies[j].0);
            let max_col = std::cmp::max(galaxies[i].1, galaxies[j].1);
            let min_col = std::cmp::min(galaxies[i].1, galaxies[j].1);
            let mut distance = ((max_row - min_row) + (max_col - min_col)) as u128;
            for exp in min_row..max_row {
                if empty_rows.contains(&exp) {
                    distance += expansion - 1;
                }
            }
            for exp in min_col..max_col {
                if empty_cols.contains(&exp) {
                    distance += expansion - 1;
                }
            }
            sum += distance;
        }
    }
    sum.to_string()
}

fn is_col_empty(matrix: &Vec<Vec<char>>, col: usize) -> bool {
    let mut is_empty = true;
    for row in 0..matrix.len() {
        if matrix[row][col] == '#' {
            is_empty = false;
            break;
        }
    }
    is_empty
}

fn is_row_empty(matrix: &Vec<Vec<char>>, row: usize) -> bool {
    let mut is_empty = true;
    for col in 0..matrix[0].len() {
        if matrix[row][col] == '#' {
            is_empty = false;
            break;
        }
    }
    is_empty
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            solve(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                2
            ),
            "374"
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            solve(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                10
            ),
            "1030"
        );
    }
}
