use std::collections::HashMap;

pub fn build_adapters(input: &str) -> Vec<u64> {
  let mut adapters: Vec<u64> = Vec::new();
  adapters.push(0);
  for l in input.lines() {
    adapters.push(l.parse::<u64>().unwrap());
  }
  adapters.sort();
  adapters.push(adapters.last().unwrap() + 3);
  adapters
}

pub fn part_one(input: &str) -> u64 {
  let adapters = build_adapters(input);
  let mut diff_1 = 0;
  let mut diff_3 = 0;
  for i in 0..adapters.len() - 1 {
    if adapters[i] + 1 == adapters[i + 1] {
      diff_1 += 1;
    } else if adapters[i] + 3 == adapters[i + 1] {
      diff_3 += 1;
    }
  }
  diff_1 * diff_3
}

pub fn find_next_adapters(
  start_i: usize,
  memo: &mut HashMap<usize, u64>,
  adapters: &Vec<u64>,
) -> u64 {
  if start_i == adapters.len() - 1 {
    return 1;
  }
  let mut sum = 0;
  for i in start_i + 1..adapters.len() {
    if adapters[start_i] + 3 >= adapters[i] {
      if let Some(val) = memo.get(&i) {
        sum += val;
        continue;
      }
      let result = find_next_adapters(i, memo, adapters);
      memo.insert(i, result);
      sum += result;
    }
  }
  sum
}

pub fn part_two(input: &str) -> u64 {
  let adapters = build_adapters(input);
  let mut memo: HashMap<usize, u64> = HashMap::new();
  find_next_adapters(0, &mut memo, &adapters)
}

#[cfg(test)]
mod day10_tests {
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
