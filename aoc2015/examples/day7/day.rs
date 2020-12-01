extern crate aoc2015;

use aoc2015::circuit::Circuit;

pub fn part_one(input: &str) -> i64 {
  let mut c = Circuit::new();
  for line in input.lines() {
    c.handle_instruction(line).unwrap();
  }
  *c.get_wire("a").unwrap() as i64
}

pub fn part_two(input: &str) -> i64 {
  println!("{}", input);
  0
}

#[cfg(test)]
mod day7_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    let input = "123 -> x\n\
    456 -> y\n\
    x AND y -> a";
    assert_eq!(part_one(input), 72);

    let input = "123 -> x\n\
    456 -> y\n\
    x OR y -> a";
    assert_eq!(part_one(input), 507);

    let input = "123 -> x\n\
    456 -> y\n\
    x LSHIFT 2 -> a";
    assert_eq!(part_one(input), 492);

    let input = "123 -> x\n\
    456 -> y\n\
    y RSHIFT 2 -> a";
    assert_eq!(part_one(input), 114);

    let input = "123 -> x\n\
    NOT x -> a";
    assert_eq!(part_one(input), 65412);

    let input = "456 -> y\n\
    NOT y -> a";
    assert_eq!(part_one(input), 65079);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
