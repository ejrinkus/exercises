use aoc_helpers::*;
use std::cmp::{min, max};

const YEAR: u32 = 2015;
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

fn calc_paper(line: &str) -> i64 {
  let dims: Vec<i64> = line.split("x")
                           .map(|d| d.parse::<i64>().unwrap())
                           .collect();
  let lw = dims[0] * dims[1];
  let lh = dims[0] * dims[2];
  let wh = dims[1] * dims[2];
  2 * (lw + lh + wh) + min(lw, min(lh, wh))

}

fn calc_ribbon(line: &str) -> i64 {
  let dims: Vec<i64> = line.split("x")
                           .map(|d| d.parse::<i64>().unwrap())
                           .collect();
  let perim = 2*dims[0]
              + 2*dims[1]
              + 2*dims[2]
              - 2*max(dims[0], max(dims[1], dims[2]));
  dims[0] * dims[1] * dims[2] + perim

}

pub fn part_one(input: &str) -> i64 {
  input.lines().map(|l| calc_paper(l)).sum()
}

pub fn part_two(input: &str) -> i64 {
  
  input.lines().map(|l| calc_ribbon(l)).sum()
}

#[cfg(test)]
mod day2_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("2x3x4"), 58);
    assert_eq!(part_one("1x1x10"), 43);
    assert_eq!(part_one("2x3x4\n1x1x10"), 101);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("2x3x4"), 34);
    assert_eq!(part_two("1x1x10"), 14);
    assert_eq!(part_two("2x3x4\n1x1x10"), 48);
  }
}
