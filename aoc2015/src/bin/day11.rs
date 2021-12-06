use aoc_helpers::*;

const YEAR: u32 = 2015;
const DAY: u32 = 11;

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

pub fn is_valid(input: &Vec<char>) -> bool {
  // Flip to true if we find a 3-straight of letters.
  let mut first_rule = false;
  // If we find a char get repeated in two adjacent positions, replace one entry
  // in this tuple.  The two entries can't be the same char.
  let mut third_rule = (0 as char, 0 as char);
  for (i, c) in input.iter().enumerate() {
    if *c == 'i' || *c == 'o' || *c == 'l' {
      // Rule 2
      return false;
    }
    if i < input.len() - 2
      && input[i + 1] == std::char::from_u32(*c as u32 + 1).unwrap()
      && input[i + 2] == std::char::from_u32(*c as u32 + 2).unwrap()
    {
      first_rule = true;
    }
    if i < input.len() - 1 && input[i + 1] == *c && third_rule.0 != *c {
      if third_rule.0 == 0 as char {
        third_rule.0 = *c;
      } else {
        third_rule.1 = *c;
      }
    }
    if first_rule && third_rule.0 > 0 as char && third_rule.1 > 0 as char {
      return true;
    }
  }
  false
}

pub fn get_next_pass(input: &mut Vec<char>) {
  for i in (0..input.len()).rev() {
    if input[i] == 'z' {
      input[i] = 'a';
    } else {
      input[i] = std::char::from_u32(input[i] as u32 + 1).unwrap();
      break;
    }
  }
}

pub fn part_one(input: &str) -> String {
  let mut password: Vec<char> = input.trim().chars().collect();
  loop {
    get_next_pass(&mut password);
    if is_valid(&password) {
      break;
    }
  }
  password.iter().collect()
}

pub fn part_two(input: &str) -> String {
  let password = part_one(input);
  part_one(&password)
}

#[cfg(test)]
mod day11_tests {
  use super::*;

  #[test]
  fn is_valid_test() {
    let mut input: Vec<char> = vec!['h', 'i', 'j', 'k', 'l', 'm', 'm', 'n'];
    assert_eq!(is_valid(&input), false);
    input = vec!['a', 'b', 'b', 'c', 'e', 'f', 'f', 'g'];
    assert_eq!(is_valid(&input), false);
  }

  #[test]
  fn get_next_pass_test() {
    assert_eq!(part_one("abcdefgh"), "abcdffaa");
    assert_eq!(part_one("ghijklmn"), "ghjaabcc");
  }
}
