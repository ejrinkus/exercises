pub fn get_seat(input: &str) -> (u32, u32, u32) {
  let mut row = 127;
  let mut col = 7;
  for (i, c) in input.chars().enumerate() {
    if i < 7 && c == 'F' {
      row -= 2_u32.pow(6 - i as u32);
    } else if i >= 7 && c == 'L' {
      col -= 2_u32.pow(9 - i as u32);
    }
  }
  (row, col, row * 8 + col)
}

pub fn part_one(input: &str) -> i64 {
  let mut highest_id = 0;
  for line in input.lines() {
    let assignment = get_seat(line);
    if assignment.2 > highest_id {
      highest_id = assignment.2;
    }
  }
  highest_id as i64
}

pub fn part_two(input: &str) -> i64 {
  let mut ids: Vec<u32> = Vec::new();
  for line in input.lines() {
    let assignment = get_seat(line);
    ids.push(assignment.2);
  }
  ids.sort();
  let mut result = -1;
  for i in 1..ids.len() - 1 {
    if ids[i] + 1 != ids[i + 1] {
      result = ids[i] as i64 + 1;
    }
  }
  result
}

#[cfg(test)]
mod day5_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(get_seat("BFFFBBFRRR"), (70, 7, 567));
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
