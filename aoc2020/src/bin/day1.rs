use aoc_helpers::*;

const YEAR: u32 = 2020;
const DAY: u32 = 1;

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
  for (i, l1) in input.lines().enumerate() {
    for (_j, l2) in input.lines().enumerate().skip(i + 1) {
      let first = l1.parse::<i64>().unwrap();
      let second = l2.parse::<i64>().unwrap();
      if first + second == 2020 {
        return first * second;
      }
    }
  }
  0
}

pub fn part_two(input: &str) -> i64 {
  for (i, l1) in input.lines().enumerate() {
    for (j, l2) in input.lines().enumerate().skip(i + 1) {
      for (_k, l3) in input.lines().enumerate().skip(j + 1) {
        let first = l1.parse::<i64>().unwrap();
        let second = l2.parse::<i64>().unwrap();
        let third = l3.parse::<i64>().unwrap();
        if first + second + third == 2020 {
          return first * second * third;
        }
      }
    }
  }
  0
}

#[cfg(test)]
mod day1_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "1721
979
366
299
675
1456"
      ),
      514579
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        "1721
979
366
299
675
1456"
      ),
      241861950
    );
  }
}
