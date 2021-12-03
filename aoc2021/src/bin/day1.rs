use aoc_helpers::*;

const YEAR: u32 = 2021;
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
  solution(input, 1)
}

pub fn part_two(input: &str) -> i64 {
  solution(input, 3)
}

fn solution(input: &str, skips: usize) -> i64 {
  let mut count = 0i64;
  input
    .lines()
    .zip(input.lines().skip(skips))
    .for_each(|(x, y)| {
      if x.parse::<i64>().unwrap() < y.parse::<i64>().unwrap() {
        count += 1;
      }
    });
  count
}

#[cfg(test)]
mod day1_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "199
200
208
210
200
207
240
269
260
263"
      ),
      7
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        "199
200
208
210
200
207
240
269
260
263"
      ),
      5
    );
  }
}
