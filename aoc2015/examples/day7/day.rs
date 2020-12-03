extern crate aoc2015;

use aoc2015::circuit::Circuit;

pub fn part_one(input: &str) -> i64 {
  let mut c = Circuit::new();
  for line in input.lines() {
    c.handle_instruction(line).unwrap();
  }
  if let Err(err) = c.resolve() {
    panic!(err);
  }
  c.get_wire("a").unwrap() as i64
}

pub fn part_two(input: &str) -> i64 {
  // Same solution, different input.
  part_one(input)
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
