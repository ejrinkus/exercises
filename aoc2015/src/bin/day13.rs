use aoc_helpers::*;

const YEAR: u32 = 2015;
const DAY: u32 = 12;

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
  0
}

pub fn part_two(input: &str) -> i64 {
  0
}

#[cfg(test)]
mod day12_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("{}"), 0);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
