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
  let mut password: Vec<char> = input.chars().collect();
  while !is_valid(&password) {
    get_next_pass(&mut password);
  }
  password.iter().collect()
}

pub fn part_two(input: &str) -> String {
  let mut password: Vec<char> = input.chars().collect();
  get_next_pass(&mut password);
  let next_pass: String = password.iter().collect();
  part_one(&next_pass)
}

#[cfg(test)]
mod day11_tests {
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
