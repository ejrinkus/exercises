use aoc_helpers::*;
use aoc2015::circuit::Circuit;
use regex::{Captures, Regex};

const YEAR: u32 = 2015;
const DAY: u32 = 7;

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
  let mut c = Circuit::new();
  for line in input.lines() {
    c.handle_instruction(line).unwrap();
  }
  if let Err(err) = c.resolve() {
    panic!("{}", err);
  }
  c.get_wire("a").unwrap() as i64
}

pub fn part_two(input: &str) -> i64 {
  // Same solution, different input.
  let re = Regex::new(r"\d+( -> b)").unwrap();
  let new_input = re.replace(input, |caps: &Captures| {
    format!("{}{}", part_one(input), &caps[1])
  });
  part_one(&new_input)
}

#[cfg(test)]
mod day7_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    let input = "123 -> x
456 -> y
x AND y -> a";
    assert_eq!(part_one(input), 72);
    println!("success");

    let input = "123 -> x
456 -> y
x OR y -> a";
    assert_eq!(part_one(input), 507);
    println!("success");

    let input = "123 -> x
456 -> y
x LSHIFT 2 -> a";
    assert_eq!(part_one(input), 492);
    println!("success");

    let input = "123 -> x
456 -> y
y RSHIFT 2 -> a";
    assert_eq!(part_one(input), 114);
    println!("success");

    let input = "123 -> x
NOT x -> a";
    assert_eq!(part_one(input), 65412);
    println!("success");

    let input = "456 -> y
NOT y -> a";
    assert_eq!(part_one(input), 65079);
    println!("success");
  }
}
