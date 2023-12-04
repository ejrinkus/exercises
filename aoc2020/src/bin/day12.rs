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
        12
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

pub fn parse_actions(input: &str) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    for line in input.lines() {
        let c: char = line.as_bytes()[0] as char;
        let val: u32 = line[1..].parse::<u32>().unwrap();
        let action = match c {
            'N' => Action::North(val),
            'S' => Action::South(val),
            'E' => Action::East(val),
            'W' => Action::West(val),
            'L' => Action::Left(val),
            'R' => Action::Right(val),
            'F' => Action::Forward(val),
            _ => panic!("invalid action: {}", line),
        };
        actions.push(action);
    }
    actions
}

pub fn follow_directions(actions: &Vec<Action>) -> (i32, i32) {
    let mut facing = Action::East(0);
    let (mut x, mut y) = (0_i32, 0_i32);
    for action in actions {
        match action {
            Action::North(val) => y += *val as i32,
            Action::South(val) => y -= *val as i32,
            Action::East(val) => x += *val as i32,
            Action::West(val) => x -= *val as i32,
            Action::Left(val) => {
                if (facing == Action::East(0) && *val == 90)
                    || (facing == Action::South(0) && *val == 180)
                    || (facing == Action::West(0) && *val == 270)
                {
                    facing = Action::North(0);
                } else if (facing == Action::West(0) && *val == 90)
                    || (facing == Action::North(0) && *val == 180)
                    || (facing == Action::East(0) && *val == 270)
                {
                    facing = Action::South(0);
                } else if (facing == Action::South(0) && *val == 90)
                    || (facing == Action::West(0) && *val == 180)
                    || (facing == Action::North(0) && *val == 270)
                {
                    facing = Action::East(0);
                } else if (facing == Action::North(0) && *val == 90)
                    || (facing == Action::East(0) && *val == 180)
                    || (facing == Action::South(0) && *val == 270)
                {
                    facing = Action::West(0);
                }
            }
            Action::Right(val) => {
                if (facing == Action::East(0) && *val == 270)
                    || (facing == Action::South(0) && *val == 180)
                    || (facing == Action::West(0) && *val == 90)
                {
                    facing = Action::North(0);
                } else if (facing == Action::West(0) && *val == 270)
                    || (facing == Action::North(0) && *val == 180)
                    || (facing == Action::East(0) && *val == 90)
                {
                    facing = Action::South(0);
                } else if (facing == Action::South(0) && *val == 270)
                    || (facing == Action::West(0) && *val == 180)
                    || (facing == Action::North(0) && *val == 90)
                {
                    facing = Action::East(0);
                } else if (facing == Action::North(0) && *val == 270)
                    || (facing == Action::East(0) && *val == 180)
                    || (facing == Action::South(0) && *val == 90)
                {
                    facing = Action::West(0);
                }
            }
            Action::Forward(val) => match facing {
                Action::North(_) => y += *val as i32,
                Action::South(_) => y -= *val as i32,
                Action::East(_) => x += *val as i32,
                Action::West(_) => x -= *val as i32,
                _ => panic!("Invalid facing: {:?}", facing),
            },
        };
    }
    (x, y)
}

pub fn move_waypoint(actions: &Vec<Action>) -> (i32, i32) {
    let (mut x, mut y) = (10_i32, 1_i32);
    let (mut shipx, mut shipy) = (0_i32, 0_i32);
    for action in actions {
        match action {
            Action::North(val) => y += *val as i32,
            Action::South(val) => y -= *val as i32,
            Action::East(val) => x += *val as i32,
            Action::West(val) => x -= *val as i32,
            Action::Left(val) => {
                if *val == 90 {
                    let tmp = y;
                    y = x;
                    x = tmp * -1;
                } else if *val == 180 {
                    x *= -1;
                    y *= -1;
                } else if *val == 270 {
                    let tmp = y;
                    y = x * -1;
                    x = tmp;
                }
            }
            Action::Right(val) => {
                if *val == 270 {
                    let tmp = y;
                    y = x;
                    x = tmp * -1;
                } else if *val == 180 {
                    x *= -1;
                    y *= -1;
                } else if *val == 90 {
                    let tmp = y;
                    y = x * -1;
                    x = tmp;
                }
            }
            Action::Forward(val) => {
                shipx += x * *val as i32;
                shipy += y * *val as i32;
            }
        };
    }
    (shipx, shipy)
}

pub fn part_one(input: &str) -> i32 {
    let actions = parse_actions(input);
    let (x, y) = follow_directions(&actions);
    println!("({}, {})", x, y);
    x.abs() + y.abs()
}

pub fn part_two(input: &str) -> i32 {
    let actions = parse_actions(input);
    let (x, y) = move_waypoint(&actions);
    println!("({}, {})", x, y);
    x.abs() + y.abs()
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "F10
N3
F7
R90
F11"
            ),
            25
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "F10
N3
F7
R90
F11"
            ),
            286
        );
    }
}
