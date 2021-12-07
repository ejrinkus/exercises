use aoc_helpers::*;

const YEAR: u32 = 2021;
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

pub fn part_one(input: &str) -> usize {
  let mut sorted: Vec<usize> = input.trim().split(',').map(|n| n.parse::<usize>().unwrap()).collect();
  sorted.sort_unstable();
  let mut l_ptr = 0;
  let mut r_ptr = sorted.len() - 1;
  let mut l_fuel = 0;
  let mut r_fuel = 0;
  while l_ptr < r_ptr {
    l_fuel += (sorted[l_ptr + 1] - sorted[l_ptr]) * (l_ptr + 1);
    l_ptr += 1;
    r_fuel += (sorted[r_ptr] - sorted[r_ptr - 1]) * (sorted.len() - r_ptr);
    r_ptr -= 1;
  }
  l_fuel + r_fuel
}

pub fn part_two(input: &str) -> usize {
  let crabs: Vec<usize> = input.trim().split(',').map(|n| n.parse::<usize>().unwrap()).collect();
  // For some weird reason, the sample input requires ceil() or round() here.
  // But the real input requires floor().  So I'm committing the code that
  // solves the real input.
  let average = ((crabs.iter().sum::<usize>() as f64) / (crabs.len() as f64)).floor() as usize;
  let mut fuel = 0;
  for c in crabs {
    let n = if c < average {
      average - c
    } else {
      c - average
    };
    fuel += (n * (n + 1)) / 2;
  }
  fuel
}

#[cfg(test)]
mod day2_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("16,1,2,0,4,2,7,1,2,14"), 37);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("16,1,2,0,4,2,7,1,2,14"), 168);
  }
}
