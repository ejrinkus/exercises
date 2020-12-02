pub fn part_one(input: &str) -> i64 {
  input.lines().fold(0, |acc, l| {
    // Each DB entry is space-delimited: <range, letter, password>
    let parts: Vec<&str> = l.split(' ').collect();
    if parts.len() != 3 {
      panic!("Malformatted password entry: {}", l);
    }

    // Get the min and max values for the range (they are inclusive).
    let mut range = parts[0].split('-');
    let min = range.next().unwrap().parse::<u8>().unwrap();
    let max = range.next().unwrap().parse::<u8>().unwrap();

    // Get the letter for validation (ignore the colon).
    let letter = parts[1].chars().next().unwrap();

    // Get the password.
    let password = parts[2];

    // Count each occurrence of the letter in the password.  If it's between
    // min and max (inclusive), then the password is valid.
    let mut count = 0;
    for c in password.chars() {
      if c == letter {
        count += 1;
      }
      if count > max {
        // Don't continue counting if we already know the password is invalid.
        return acc;
      }
    }
    if count >= min {
      // Since we have the fail-fast check, we know that count must be <= max.
      return acc + 1;
    }
    acc
  })
}

pub fn part_two(input: &str) -> i64 {
  input.lines().fold(0, |acc, l| {
    // Each DB entry is space-delimited: <range, letter, password>
    let parts: Vec<&str> = l.split(' ').collect();
    if parts.len() != 3 {
      panic!("Malformatted password entry: {}", l);
    }

    // Get the min and max values for the range (they are inclusive).
    let mut range = parts[0].split('-');
    let min = range.next().unwrap().parse::<usize>().unwrap();
    let max = range.next().unwrap().parse::<usize>().unwrap();

    // Get the letter for validation (ignore the colon).
    let letter = parts[1].chars().next().unwrap();

    // Get the password.
    let password = parts[2];

    // Indices are not zero indexed here, so we need to skip min-1 to place
    // ourselves right before the letter we're interested in.
    let mut found_letter = false;
    let mut iter = password.chars();
    if iter.nth(min - 1) == Some(letter) {
      found_letter = true;
    }
    if iter.nth(max - min - 1) == Some(letter) {
      if found_letter {
        // Letter occurred in both indices, password is invalid.
        return acc;
      } else {
        // Letter only found in the second index, password valid.
        return acc + 1;
      }
    }
    if found_letter {
      // Letter only found in the first index, password valid.
      return acc + 1;
    }
    // Letter not found in either index, password invalid.
    acc
  })
}

#[cfg(test)]
mod day2_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"
      ),
      2
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"
      ),
      1
    );
  }
}
