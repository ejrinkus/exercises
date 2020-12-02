pub fn part_one(input: &str) -> i64 {
  input.lines().fold(0, |acc, l| {
    let parts: Vec<&str> = l.split(' ').collect();
    if parts.len() != 3 {
      panic!("Malformatted password entry: {}", l);
    }
    let mut range = parts[0].split('-');
    let min = range.next().unwrap().parse::<u8>().unwrap();
    let max = range.next().unwrap().parse::<u8>().unwrap();
    let letter = parts[1].trim_end_matches(':').parse::<char>().unwrap();
    let password = parts[2].to_owned();
    let mut count = 0;
    for c in password.chars() {
      if c == letter {
        count += 1;
      }
    }
    if count >= min && count <= max {
      return acc + 1;
    }
    acc
  })
}

pub fn part_two(input: &str) -> i64 {
  println!("{}", input);
  0
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
    assert_eq!(part_two(""), 0);
  }
}
