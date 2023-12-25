use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}

struct State {
    row: usize,
    col: usize,
    heat: u32,
    straight: u8,
    dir: Direction,
}

impl State {
    fn new(dir: Direction) -> Self {
        Self {
            row: 0,
            col: 0,
            heat: 0,
            straight: 0,
            dir,
        }
    }
}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        17
    }

    fn part_one(&self, input: &str) -> String {
        let city = parse_num_matrix(input);
        let mut heat = city.clone();
        for r in 0..heat.len() {
            for c in 0..heat[r].len() {
                heat[r][c] = -1;
            }
        }

        let mut expansion: Vec<State> = Vec::new();
        expansion.push(State::new(Direction::South));
        expansion.push(State::new(Direction::East));

        while expansion.len() > 0 {}

        "".to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        "".to_string()
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
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            "102"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            ""
        );
    }
}
