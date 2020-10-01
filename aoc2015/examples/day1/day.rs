pub fn part_one(input: &str) -> i64 {
  let pos = input.chars().filter(|x| *x == '(').count() as i64;
  let neg = input.chars().filter(|x| *x == ')').count() as i64;
  pos - neg
}

pub fn part_two(input: &str) -> usize {
  let mut floor: i64 = 0;
  let mut count: usize = 1;
  for c in input.chars() {
    if c == '(' {
      floor += 1;
    } else if c == ')' {
      floor -= 1;
    }
    if floor < 0 {
      break;
    }
    count += 1;
  }
  count
}

#[cfg(test)]
mod day1_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("(())"), 0);
    assert_eq!(part_one("()()"), 0);
    assert_eq!(part_one("((("), 3);
    assert_eq!(part_one("(()(()("), 3);
    assert_eq!(part_one("))((((("), 3);
    assert_eq!(part_one("())"), -1);
    assert_eq!(part_one("))("), -1);
    assert_eq!(part_one(")))"), -3);
    assert_eq!(part_one(")())())"), -3);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(")"), 1);
    assert_eq!(part_two("()())"), 5);
  }
}