use aoc_helpers::*;

const YEAR: u32 = 2021;
const DAY: u32 = 2;

fn main() {
  let input = get_input(YEAR, DAY);
  if prompt_for_part(1) {
    let result = part_one(&input);
    println!("Part one: {}", result);
    if prompt_to_submit() {
      println!("{}", submit_answer(YEAR, DAY, 1, &result.to_string()));
    }
  }
  if prompt_for_part(2) {
    let result = part_two(&input);
    println!("Part two: {}", result);
    if prompt_to_submit() {
      println!("{}", submit_answer(YEAR, DAY, 2, &result.to_string()));
    }
  }
}

pub fn part_one(input: &str) -> i64 {
  let mut horizontal = 0i64;
  let mut depth = 0i64;
  for line in input.lines() {
    let mut pieces = line.split(" ");
    let direction = pieces.next().unwrap();
    let val = pieces.next().unwrap().parse::<i64>().unwrap();
    if direction == "forward" {
      horizontal += val;
    } else if direction == "down" {
      depth += val;
    } else if direction == "up" {
      depth -= val;
    }
  }
  horizontal * depth
}

pub fn part_two(input: &str) -> i64 {
  let mut horizontal = 0i64;
  let mut aim = 0i64;
  let mut depth = 0i64;
  for line in input.lines() {
    let mut pieces = line.split(" ");
    let direction = pieces.next().unwrap();
    let val = pieces.next().unwrap().parse::<i64>().unwrap();
    if direction == "forward" {
      horizontal += val;
      depth += val * aim;
    } else if direction == "down" {
      aim += val;
    } else if direction == "up" {
      aim -= val;
    }
  }
  horizontal * depth
}

#[cfg(test)]
mod day2_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one(""), 0);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
