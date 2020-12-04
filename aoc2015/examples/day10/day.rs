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
  let mut result = input.to_string();
  for _i in 0..40 {
    result = look_and_say(&result);
  }
  result.len()
}

pub fn part_two(input: &str) -> usize {
  let mut result = input.to_string();
  for _i in 0..50 {
    result = look_and_say(&result);
  }
  result.len()
}

#[cfg(test)]
mod day10_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one(""), 0);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
