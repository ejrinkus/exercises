use std::collections::HashSet;

pub fn part_one(input: &str) -> i64 {
  let (mut x, mut y): (i64, i64) = (0, 0);
  let mut visited = HashSet::new();
  let mut total: i64 = 1;
  visited.insert((x, y));
  for c in input.chars() {
    match c {
      '^' => {
        y += 1;
      },
      '>' => {
        x += 1;
      },
      'v' => {
        y -= 1;
      },
      '<' => {
        x -= 1;
      },
      _ => {
        panic!("unexpected direction!");
      }
    }
    if !visited.contains(&(x, y)) {
      total += 1;
      visited.insert((x, y));
    }
  }
  total
}

pub fn part_two(input: &str) -> i64 {
  let (mut x, mut y): (i64, i64) = (0, 0);
  let (mut robo_x, mut robo_y): (i64, i64) = (0, 0);
  let mut visited = HashSet::new();
  let mut total: i64 = 1;
  visited.insert((x, y));
  let mut move_santa = true;
  for c in input.chars() {
    match c {
      '^' => {
        if move_santa {
          y += 1;
        } else {
          robo_y += 1;
        }
      },
      '>' => {
        if move_santa {
          x += 1;
        } else {
          robo_x += 1;
        }
      },
      'v' => {
        if move_santa {
          y -= 1;
        } else {
          robo_y -= 1;
        }
      },
      '<' => {
        if move_santa {
          x -= 1;
        } else {
          robo_x -= 1;
        }
      },
      _ => {
        panic!("unexpected direction!");
      }
    }
    if move_santa && !visited.contains(&(x, y)) {
      total += 1;
      visited.insert((x, y));
    } else if !move_santa && !visited.contains(&(robo_x, robo_y)) {
      total += 1;
      visited.insert((robo_x, robo_y));
    }
    move_santa = !move_santa;
  }
  total
}

#[cfg(test)]
mod day3_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one(">"), 2);
    assert_eq!(part_one("^>v<"), 4);
    assert_eq!(part_one("^>vv"), 5);
    assert_eq!(part_one("^v^v^v^v^v"), 2);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("^v"), 3);
    assert_eq!(part_two("^>v<"), 3);
    assert_eq!(part_two("^v^v^v^v^v"), 11);
  }
}
