use aoc_helpers::*;
use std::collections::HashMap;

const YEAR: u32 = 2020;
const DAY: u32 = 23;

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

pub fn build_cups(input: &str) -> (u64, u64, HashMap<u64, u64>) {
  let mut cups: HashMap<u64, u64> = HashMap::new();
  let mut prev_label = 0_u64;
  let mut first_label = 0_u64;
  let mut max = 0_u64;
  for l in input.chars() {
    let label = l.to_digit(10).unwrap() as u64;
    if label > max {
      max = label;
    }
    if prev_label != 0 {
      cups.insert(prev_label, label);
    } else {
      first_label = label;
    }
    prev_label = label;
  }
  cups.insert(prev_label, first_label);
  (first_label, max, cups)
}

pub fn make_move(start: u64, max: u64, cups: &mut HashMap<u64, u64>) -> u64 {
  let first = *cups.get(&start).unwrap();
  let second = *cups.get(&first).unwrap();
  let third = *cups.get(&second).unwrap();
  let fourth = *cups.get(&third).unwrap();
  let mut dest = start - 1;
  while dest == first || dest == second || dest == third || dest == 0 {
    if dest == 0 {
      dest = max;
    } else {
      dest -= 1;
    }
  }
  let dest_next = *cups.get(&dest).unwrap();

  *cups.get_mut(&start).unwrap() = fourth;
  *cups.get_mut(&third).unwrap() = dest_next;
  *cups.get_mut(&dest).unwrap() = first;
  fourth
}

pub fn state_to_val(cups: &HashMap<u64, u64>) -> u64 {
  let mut result = 0;
  let mut curr = cups.get(&1).unwrap();
  while *curr != 1 {
    result *= 10;
    result += curr;
    curr = cups.get(&curr).unwrap();
  }
  result
}

pub fn part_one(input: &str) -> u64 {
  let (first_label, max, mut cups) = build_cups(input);
  let mut curr_label = first_label;
  for _ in 0..100 {
    curr_label = make_move(curr_label, max, &mut cups);
  }
  state_to_val(&cups)
}

pub fn part_two(input: &str) -> u64 {
  let (first_label, _, mut cups) = build_cups(input);
  let mut last_label = first_label;
  while *cups.get(&last_label).unwrap() != first_label {
    last_label = *cups.get(&last_label).unwrap();
  }
  let max = 1000000_u64;
  for i in 10..=max {
    cups.insert(last_label, i);
    last_label = i;
  }
  cups.insert(max, first_label);
  let mut curr_label = first_label;
  for _ in 0..10000000_u64 {
    curr_label = make_move(curr_label, max, &mut cups);
  }
  let first = *cups.get(&1).unwrap();
  let second = *cups.get(&first).unwrap();
  first * second
}

#[cfg(test)]
mod day23_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("389125467"), 67384529);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("389125467"), 149245887792);
  }
}
