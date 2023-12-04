use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::collections::HashSet;
use std::vec::Vec;

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
        9
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

struct HeightMap {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl HeightMap {
    pub fn new(input: &str) -> HeightMap {
        let mut map = HeightMap {
            grid: Vec::new(),
            width: 0,
            height: 0,
        };
        for line in input.lines() {
            map.add_row(line);
        }
        map
    }

    fn add_row(&mut self, row: &str) {
        if self.width == 0 {
            self.width = row.len()
        }
        let rowvec: Vec<u8> = row.chars().map(|c| c as u8 - 48).collect();
        self.grid.push(rowvec);
        self.height += 1;
    }

    pub fn get_lowpoints(&self) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let val = self.grid[y][x];
                if x > 0 && self.grid[y][x - 1] <= val {
                    continue;
                }
                if y > 0 && self.grid[y - 1][x] <= val {
                    continue;
                }
                if x < self.width - 1 && self.grid[y][x + 1] <= val {
                    continue;
                }
                if y < self.height - 1 && self.grid[y + 1][x] <= val {
                    continue;
                }
                points.push((x, y));
            }
        }
        points
    }

    pub fn calc_risk(&self) -> u32 {
        self.get_lowpoints()
            .iter()
            .fold(0u32, |acc, (x, y)| acc + 1 + self.grid[*y][*x] as u32)
    }

    pub fn find_basin_size(&self, start_x: usize, start_y: usize) -> usize {
        let mut size = 0usize;
        let mut basin: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let bottom = self.grid[start_y][start_x];
        basin.push((start_x, start_y));
        while !basin.is_empty() {
            let (x, y) = basin.pop().unwrap();
            let val = self.grid[y][x];
            if val == 9 || val < bottom || visited.contains(&(x, y)) {
                continue;
            }
            size += 1;
            visited.insert((x, y));
            if x > 0 {
                basin.push((x - 1, y));
            }
            if y > 0 {
                basin.push((x, y - 1));
            }
            if x < self.width - 1 {
                basin.push((x + 1, y));
            }
            if y < self.height - 1 {
                basin.push((x, y + 1));
            }
        }
        size
    }
}

pub fn part_one(input: &str) -> u32 {
    let map = HeightMap::new(input);
    map.calc_risk()
}

pub fn part_two(input: &str) -> u64 {
    let map = HeightMap::new(input);
    let mut sizes: [usize; 3] = [0; 3];
    for (x, y) in map.get_lowpoints() {
        let mut size = map.find_basin_size(x, y);
        for i in (0..3).rev() {
            if size > sizes[i] {
                let temp = sizes[i];
                sizes[i] = size;
                size = temp;
            }
        }
    }
    sizes.iter().fold(1u64, |acc, s| acc * *s as u64)
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            15
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            1134
        );
    }
}
