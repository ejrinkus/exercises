use std::collections::HashSet;

pub fn get_sums(window: &[u64]) -> HashSet<u64> {
  let mut sum_set: HashSet<u64> = HashSet::new();
  for i in 0..window.len() - 1 {
    for j in i + 1..window.len() {
      sum_set.insert(window[i] + window[j]);
    }
  }
  sum_set
}

pub fn find_failure(numbers: &Vec<u64>) -> u64 {
  for i in 25..numbers.len() {
    let sum_set = get_sums(&numbers[i - 25..i]);
    if !sum_set.contains(&numbers[i]) {
      return numbers[i];
    }
  }
  0
}

pub fn part_one(input: &str) -> u64 {
  let numbers: Vec<u64> = input
    .split('\n')
    .map(|n| n.parse::<u64>().unwrap())
    .collect();
  find_failure(&numbers)
}

pub fn find_sum(numbers: &Vec<u64>, invalid: u64) -> u64 {
  let mut start = 0;
  let mut end = 1;
  let mut sum = numbers[start] + numbers[end];
  loop {
    if sum > invalid {
      sum -= numbers[start];
      start += 1;
      if start == end && end != numbers.len() - 1 {
        end += 1;
        sum += numbers[end];
      }
    } else if sum < invalid && end != numbers.len() - 1 {
      end += 1;
      sum += numbers[end];
    } else {
      println!("{}, {}", start, end);
      return numbers[start..end + 1].iter().max().unwrap()
        + numbers[start..end + 1].iter().min().unwrap();
    }
    if start == numbers.len() - 1 && start == end {
      break;
    }
  }
  0
}

pub fn part_two(input: &str) -> u64 {
  let numbers: Vec<u64> = input
    .split('\n')
    .map(|n| n.parse::<u64>().unwrap())
    .collect();
  let invalid = find_failure(&numbers);
  find_sum(&numbers, invalid)
}

#[cfg(test)]
mod day9_tests {
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
