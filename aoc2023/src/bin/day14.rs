use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::collections::HashMap;

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
        14
    }

    fn part_one(&self, input: &str) -> String {
        let mut grid = parse_matrix(input);
        tilt_north(&mut grid);
        calc_load(&grid).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut grid = parse_matrix(input);

        let mut memo: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
        memo.insert(grid.clone(), 0);
        let mut loop_start = 0;
        let mut loop_size = 0;
        for i in 0..1000000000 {
            tilt_north(&mut grid);
            tilt_west(&mut grid);
            tilt_south(&mut grid);
            tilt_east(&mut grid);
            if memo.contains_key(&grid) {
                loop_start = *memo.get(&grid).unwrap();
                loop_size = i - loop_start;
                break;
            }
            memo.insert(grid.clone(), i + 1);
        }

        let end = ((1000000000 - loop_start) % (loop_size + 1)) + loop_start;
        for (g, i) in memo.iter() {
            if *i == end {
                return calc_load(g).to_string();
            }
        }
        calc_load(&grid).to_string()
    }
}

fn calc_load(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0usize;
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'O' {
                total += grid.len() - row;
            }
        }
    }
    total
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut next = 0;
        for row in 0..grid.len() {
            if grid[row][col] == 'O' {
                grid[row][col] = '.';
                grid[next][col] = 'O';
                next += 1;
            } else if grid[row][col] == '#' {
                next = row + 1;
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut next = grid.len() - 1;
        for row in (0..grid.len()).rev() {
            if grid[row][col] == 'O' {
                grid[row][col] = '.';
                grid[next][col] = 'O';
                if next > 0 {
                    next -= 1;
                }
            } else if grid[row][col] == '#' && row != 0 {
                next = row - 1;
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        let mut next = 0;
        for col in 0..grid[0].len() {
            if grid[row][col] == 'O' {
                grid[row][col] = '.';
                grid[row][next] = 'O';
                next += 1;
            } else if grid[row][col] == '#' {
                next = col + 1;
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        let mut next = grid[0].len() - 1;
        for col in (0..grid[0].len()).rev() {
            if grid[row][col] == 'O' {
                grid[row][col] = '.';
                grid[row][next] = 'O';
                if next > 0 {
                    next -= 1;
                }
            } else if grid[row][col] == '#' && col != 0 {
                next = col - 1;
            }
        }
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            "136"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            "64"
        );
    }
}
