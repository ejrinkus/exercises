use aoc_helpers::*;

const YEAR: u32 = 2015;
const DAY: u32 = 10;

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

pub fn look_and_say(input: &str) -> String {
  let mut output = String::with_capacity(input.len() * 2);
  let mut curr_char = input.chars().next().unwrap();
  let mut last_i = 0;
  for (i, c) in input.chars().enumerate() {
    if c == curr_char {
      continue;
    }
    let diff = i - last_i;
    output.push_str(&diff.to_string());
    output.push(curr_char);
    curr_char = c;
    last_i = i
  }
  output = format!("{}{}{}", output, input.len() - last_i, curr_char);
  output
}

pub fn part_one(input: &str) -> usize {
  let mut result = input.trim().to_string();
  for _i in 0..40 {
    result = look_and_say(&result);
  }
  result.len()
}

pub fn part_two(input: &str) -> usize {
  let mut result = input.trim().to_string();
  for _i in 0..50 {
    result = look_and_say(&result);
  }
  result.len()
}

#[cfg(test)]
mod day10_tests {
  use super::*;

  #[test]
  fn look_and_say_test() {
    let mut result = "1".to_string();
    for _i in 0..5 {
      result = look_and_say(&result);
    }
    assert_eq!(result.len(), 6);
  }
}
