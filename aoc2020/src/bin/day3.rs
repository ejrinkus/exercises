use aoc_helpers::*;

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
        3
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> i64 {
    check_slope(input, 3, 1)
}

pub fn part_two(input: &str) -> i64 {
    check_slope(input, 1, 1)
        * check_slope(input, 3, 1)
        * check_slope(input, 5, 1)
        * check_slope(input, 7, 1)
        * check_slope(input, 1, 2)
}

pub fn check_slope(input: &str, right: usize, down: usize) -> i64 {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut step = 1;
    let mut count = 0;
    loop {
        // Figure out the row and column we need to seek to.
        let row_i = (step * down) % height;
        let col_i = (step * right) % width;

        // Seek to the correct position.
        let row = input.lines().skip(row_i).next().unwrap();
        let pos = row.chars().skip(col_i).next().unwrap();
        if pos == '#' {
            count += 1;
        }
        if row_i == height - 1 {
            return count;
        }
        step += 1;
    }
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            ),
            7
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            ),
            336
        );
    }
}
