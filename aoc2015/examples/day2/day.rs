pub fn part_one(input: &str) -> i64 {
  let mut total: i64 = 0;
  for line in input.lines() {
    let dims: Vec<i64> = line.split('x')
      .map(|x| x.trim().parse::<i64>().unwrap())
      .collect();

    total += (2 * dims[0] * dims[1]) +
             (2 * dims[0] * dims[2]) +
             (2 * dims[1] * dims[2]);

    if dims[0] >= dims[1] && dims[0] >= dims[2] {
      total += dims[1] * dims[2];
    } else if dims[1] >= dims[0] && dims[1] >= dims[2] {
      total += dims[0] * dims[2];
    } else if dims[2] >= dims[0] && dims[2] >= dims[1] {
      total += dims[0] * dims[1];
    }
  }
  total
}

pub fn part_two(input: &str) -> i64 {
  let mut total: i64 = 0;
  for line in input.lines() {
    let dims: Vec<i64> = line.split('x')
      .map(|x| x.trim().parse::<i64>().unwrap())
      .collect();

    total += dims[0] * dims[1] * dims[2];

    if dims[0] >= dims[1] && dims[0] >= dims[2] {
      total += (2 * dims[1]) + (2 * dims[2]);
    } else if dims[1] >= dims[0] && dims[1] >= dims[2] {
      total += (2 * dims[0]) + (2 * dims[2]);
    } else if dims[2] >= dims[0] && dims[2] >= dims[1] {
      total += (2 * dims[0]) + (2 * dims[1]);
    }
  }
  total
}

#[cfg(test)]
mod day2_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("2x3x4"), 58);
    assert_eq!(part_one("1x1x10"), 43);
    assert_eq!(part_one("2x3x4\n1x1x10"), 101);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("2x3x4"), 34);
    assert_eq!(part_two("1x1x10"), 14);
    assert_eq!(part_two("2x3x4\n1x1x10"), 48);
  }
}
