use aoc_helpers::*

fn main() {
  let input = get_input(2021, 10);
  if prompt_for_part(1) {
    println!("Part one: {}", part_one(&input));
  }
  if prompt_for_part(2) {
    println!("Part two: {}", part_two(&input));
  }
};

pub fn part_one(_input: &str) -> i64 {
  0
}

pub fn part_two(_input: &str) -> i64 {
  0
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
