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
        16
    }

    fn part_one(&self, input: &str) -> String {
        let mirrors = parse_matrix(input);
        let mut energized: Vec<Vec<HashSet<Direction>>> = Vec::with_capacity(mirrors.len());
        for row in mirrors.iter() {
            let mut en_row: Vec<HashSet<Direction>> = Vec::with_capacity(row.len());
            for _ in row.iter() {
                en_row.push(HashSet::new());
            }
            energized.push(en_row);
        }

        trace(&mirrors, &mut energized, (0, 0), Direction::EAST);
        let count = count_energized(&energized);
        count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mirrors = parse_matrix(input);
        let mut energized: Vec<Vec<HashSet<Direction>>> = Vec::with_capacity(mirrors.len());
        for row in mirrors.iter() {
            let mut en_row: Vec<HashSet<Direction>> = Vec::with_capacity(row.len());
            for _ in row.iter() {
                en_row.push(HashSet::new());
            }
            energized.push(en_row);
        }

        let mut max = 0;
        for i in 0..mirrors.len() {
            let mut e = energized.clone();
            trace(&mirrors, &mut e, (0, i), Direction::SOUTH);
            let mut count = count_energized(&e);
            if count > max {
                max = count;
            }

            e = energized.clone();
            trace(&mirrors, &mut e, (mirrors.len() - 1, i), Direction::NORTH);
            count = count_energized(&e);
            if count > max {
                max = count;
            }

            e = energized.clone();
            trace(&mirrors, &mut e, (i, 0), Direction::EAST);
            count = count_energized(&e);
            if count > max {
                max = count;
            }

            e = energized.clone();
            trace(&mirrors, &mut e, (i, mirrors.len() - 1), Direction::WEST);
            count = count_energized(&e);
            if count > max {
                max = count;
            }
        }

        max.to_string()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Copy, Clone, Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn trace(
    mirrors: &Matrix,
    energized: &mut Vec<Vec<HashSet<Direction>>>,
    start: (usize, usize),
    dir: Direction,
) {
    let mut expansion: Vec<(usize, usize, Direction)> = Vec::new();
    expansion.push((start.0, start.1, dir));

    while let Some((r, c, dir)) = expansion.pop() {
        if energized[r][c].contains(&dir) {
            continue;
        }
        energized[r][c].insert(dir);

        let mut next_dir = dir;
        if mirrors[r][c] == '/' {
            next_dir = match dir {
                Direction::EAST => Direction::NORTH,
                Direction::NORTH => Direction::EAST,
                Direction::SOUTH => Direction::WEST,
                Direction::WEST => Direction::SOUTH,
            }
        } else if mirrors[r][c] == '\\' {
            next_dir = match dir {
                Direction::EAST => Direction::SOUTH,
                Direction::NORTH => Direction::WEST,
                Direction::SOUTH => Direction::EAST,
                Direction::WEST => Direction::NORTH,
            }
        } else if mirrors[r][c] == '|' {
            if dir == Direction::EAST || dir == Direction::WEST {
                if r > 0 {
                    expansion.push((r - 1, c, Direction::NORTH));
                }
                if r < mirrors.len() - 1 {
                    expansion.push((r + 1, c, Direction::SOUTH));
                }
                continue;
            }
        } else if mirrors[r][c] == '-' {
            if dir == Direction::NORTH || dir == Direction::SOUTH {
                if c > 0 {
                    expansion.push((r, c - 1, Direction::WEST));
                }
                if c < mirrors[r].len() - 1 {
                    expansion.push((r, c + 1, Direction::EAST));
                }
                continue;
            }
        }

        // Update coords
        match next_dir {
            Direction::EAST => {
                if c == mirrors[r].len() - 1 {
                    continue;
                }
                expansion.push((r, c + 1, next_dir));
            }
            Direction::NORTH => {
                if r == 0 {
                    continue;
                }
                expansion.push((r - 1, c, next_dir));
            }
            Direction::SOUTH => {
                if r == mirrors.len() - 1 {
                    continue;
                }
                expansion.push((r + 1, c, next_dir));
            }
            Direction::WEST => {
                if c == 0 {
                    continue;
                }
                expansion.push((r, c - 1, next_dir));
            }
        }
    }
}

fn count_energized(energized: &Vec<Vec<HashSet<Direction>>>) -> usize {
    let mut count = 0;
    for row in energized {
        for space in row {
            if space.len() > 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            "46"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            "51"
        );
    }
}
