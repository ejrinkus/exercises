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
        13
    }

    fn part_one(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut sum = 0i32;
        loop {
            let matrix = matrix_from_lines(&mut lines);
            if matrix.len() <= 0 {
                break;
            }
            let mut result = find_vertical_reflection(&matrix) + 1;
            if result == 0 {
                result = 100 * (find_horizontal_reflection(&matrix) + 1);
            }
            if result == 0 {
                panic!("Couldn't find a reflection line");
            }
            sum += result;
        }
        sum.to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        "".to_string()
    }
}

fn find_vertical_reflection(matrix: &Vec<Vec<char>>) -> i32 {
    let last_col = matrix[0].len() - 1;
    // Either the left or right column _must_ be included in the reflection set.
    // Start with the left.
    let mut right = last_col;
    while right > 0 {
        if col_range_is_reflection(matrix, 0, right) {
            return (right / 2) as i32;
        }
        right -= 1;
    }

    // If we didn't find a reflection that includes the left column, try again from
    // the right column.
    let mut left = 0;
    while left < last_col {
        if col_range_is_reflection(matrix, left, last_col) {
            return (((last_col - left) / 2) + left) as i32;
        }
        left += 1;
    }
    return -1;
}

fn col_range_is_reflection(matrix: &Vec<Vec<char>>, mut left: usize, mut right: usize) -> bool {
    if (right - left) % 2 == 0 {
        // This means that the "center" of this range is a single column.
        // The center of a reflection must be _between_ two columns though, so
        // this range cannot be valid.
        return false;
    }
    while left < right {
        if !columns_equal(matrix, left, right) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn columns_equal(matrix: &Vec<Vec<char>>, left: usize, right: usize) -> bool {
    for row in 0..matrix.len() {
        if matrix[row][left] != matrix[row][right] {
            return false;
        }
    }
    true
}

fn find_horizontal_reflection(matrix: &Vec<Vec<char>>) -> i32 {
    let last_row = matrix.len() - 1;
    // Either the top or bototm column _must_ be included in the reflection set.
    // Start with the top.
    let mut bottom = last_row;
    while bottom > 0 {
        if row_range_is_reflection(matrix, 0, bottom) {
            return (bottom / 2) as i32;
        }
        bottom -= 1;
    }

    // If we didn't find a reflection that includes the top row, try again from
    // the bottom row.
    let mut top = 0;
    while top < last_row {
        if row_range_is_reflection(matrix, top, last_row) {
            return (((last_row - top) / 2) + top) as i32;
        }
        top += 1;
    }
    return -1;
}

fn row_range_is_reflection(matrix: &Vec<Vec<char>>, mut top: usize, mut bottom: usize) -> bool {
    if (bottom - top) % 2 == 0 {
        // This means that the "center" of this range is a single row.
        // The center of a reflection must be _between_ two rows though, so
        // this range cannot be valid.
        return false;
    }
    while top < bottom {
        if !rows_equal(matrix, top, bottom) {
            return false;
        }
        top += 1;
        bottom -= 1;
    }
    true
}

fn rows_equal(matrix: &Vec<Vec<char>>, top: usize, bottom: usize) -> bool {
    for col in 0..matrix[0].len() {
        if matrix[top][col] != matrix[bottom][col] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            "405");
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(solution.part_two(""), "");
    }
}
