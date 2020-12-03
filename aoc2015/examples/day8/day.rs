pub fn part_one(input: &str) -> i64 {
  let mut code_count = 0;
  let mut mem_count = 0;
  for line in input.lines() {
    code_count += line.len();
    let mut chars_iter = line.chars();
    while let Some(c) = chars_iter.next() {
      if c == '\\' {
        let c = chars_iter.next().unwrap();
        if c == '\"' || c == '\\' {
          mem_count += 1;
        } else if c == 'x' {
          // Hex character, skip the next two characters.
          chars_iter.next();
          chars_iter.next();
          mem_count += 1;
        } else {
          panic!("unexpected escape sequence: \\{}", c);
        }
      } else if c != '\"' {
        mem_count += 1;
      }
    }
  }
  code_count as i64 - mem_count as i64
}

pub fn part_two(input: &str) -> i64 {
  let mut code_count = 0;
  let mut enc_count = 0;
  for line in input.lines() {
    let mut length = line.len();
    code_count += length;
    length += 2; // For the new pair of quotes to surround the encoded string.
    for c in line.chars() {
      if c == '\\' {
        length += 1; // To escape this backslash.
      } else if c == '\"' {
        length += 1; // to escape this quote.
      }
    }
    enc_count += length;
  }
  enc_count as i64 - code_count as i64
}

#[cfg(test)]
mod day8_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\""
      ),
      12
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\""
      ),
      19
    );
  }
}
