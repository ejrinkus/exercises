use aoc_helpers::*;

const YEAR: u32 = 2020;
const DAY: u32 = 13;

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

pub fn part_one(input: &str) -> u64 {
  let mut lines = input.lines();
  let depart = lines.next().unwrap().parse::<u64>().unwrap();
  let ids = lines.next().unwrap().split(',');
  let mut min_id = 0_u64;
  let mut min_diff = u64::MAX;
  for id in ids {
    if id == "x" {
      continue;
    }
    let id_num = id.parse::<u64>().unwrap();
    if depart % id_num == 0 {
      return 0; // There's a bus that leaves at the perfect time.
    }
    let id_diff = (((depart / id_num) * id_num) + id_num) - depart;
    if id_diff < min_diff {
      min_id = id_num;
      min_diff = id_diff;
    }
  }
  min_id * min_diff
}

// The solution for IDs a and b is some number x in the range 0..lcm, where lcm
// is the lowest common multiple of a and b.  Additional solutions can be found
// using the formula lcm*i + x, where i is any integer.
pub fn find_ts(offset: u64, id: u64, base: u64, prev_lcm: u64) -> u64 {
  let mut maybe_ts = base;
  loop {
    if (maybe_ts + offset) % id == 0 {
      return maybe_ts;
    }
    maybe_ts += prev_lcm;
  }
}

pub fn part_two(input: &str) -> u64 {
  let mut lines = input.lines();
  lines.next();
  let ids = lines.next().unwrap().split(',');
  let mut enumerated_ids: Vec<(u64, u64)> = Vec::new();
  for (offset, id) in ids.enumerate() {
    if id == "x" {
      continue;
    }
    enumerated_ids.push((offset as u64, id.parse::<u64>().unwrap()));
  }
  enumerated_ids.sort_by(|a, b| a.1.cmp(&b.1));
  let mut timestamp = 1;
  let mut prev_lcm = 1;
  for (o, id) in enumerated_ids {
    timestamp = find_ts(o, id, timestamp, prev_lcm);
    prev_lcm *= id;
  }
  timestamp
}

#[cfg(test)]
mod day13_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "939
7,13,x,x,59,x,31,19"
      ),
      295
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        "939
7,13,x,x,59,x,31,19"
      ),
      1068781
    );
  }
}