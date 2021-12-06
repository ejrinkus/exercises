use aoc_helpers::*;
use md5::*;

const YEAR: u32 = 2015;
const DAY: u32 = 4;

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
  find_secret(input, 5)
}

pub fn part_two(input: &str) -> i64 {
  find_secret(input, 6)
}

pub fn validate(digest: &Digest, zeroes: u8) -> bool {
  if zeroes > 16 {
    return false;
  }
  let limit = zeroes / 2;
  for i in 0..limit {
    if digest[i as usize] != 0 {
      return false;
    }
  }
  if zeroes % 2 == 1 && digest[limit as usize] >= 16 {
    return false;
  }
  return true;
}

pub fn find_secret(input: &str, zeroes: u8) -> i64 {
  if zeroes > 16 {
    return -1;
  }
  let mut i: i64 = 0;
  loop {
    i += 1;
    let key = format!("{}{}", input.trim(), i);
    let digest = md5::compute(key);
    if validate(&digest, zeroes) {
      return i;
    }
  }
}

#[cfg(test)]
mod day4_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("abcdef"), 609043);
    assert_eq!(part_one("pqrstuv"), 1048970);
  }
}
