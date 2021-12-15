use aoc_helpers::*;
use std::collections::HashMap;
use std::vec::Vec;

const YEAR: u32 = 2021;
const DAY: u32 = 14;

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

struct Polymerizer {
  state: Vec<u8>,
  rules: HashMap<(u8, u8), u8>,
  pairs: HashMap<(u8, u8), u64>
}

impl Polymerizer {
  pub fn new(input: &str) -> Polymerizer {
    let mut lines = input.lines();
    let state = lines.next().unwrap().as_bytes().to_vec();
    lines.next();

    let mut rules: HashMap<(u8, u8), u8> = HashMap::new();
    while let Some(l) = lines.next() {
      let mut parts = l.split(" -> ");
      let pair = parts.next().unwrap().as_bytes();
      rules.insert((pair[0], pair[1]), parts.next().unwrap().as_bytes()[0]);
    }

    let mut pairs: HashMap<(u8, u8), u64> = HashMap::new();
    for i in 1..state.len() {
      *pairs.entry((state[i-1], state[i])).or_insert(0) += 1;
    }

    Polymerizer {
      state: state,
      rules: rules,
      pairs: pairs
    }
  }

  fn run_once(&mut self) {
    let mut new_pairs: HashMap<(u8, u8), u64> = HashMap::new();
    
    for (k, v) in &self.pairs {
      let middle = self.rules.get(k).unwrap();
      *new_pairs.entry((k.0, *middle)).or_insert(0) += v;
      *new_pairs.entry((*middle, k.1)).or_insert(0) += v;
    }

    self.pairs = new_pairs;
  }

  pub fn run(&mut self, steps: u64) {
    for _i in 0..steps {
      self.run_once();
    }
  }

  pub fn get_min_max_diff(&self) -> u64 {
    let mut counts: HashMap<u8, u64> = HashMap::new();
    for ((k1, _k2), v) in &self.pairs {
      // Just use the first char in each pair to avoid double counting.
      *counts.entry(*k1).or_insert(0) += v;
    }
    // Since we only used the first char in each pair above, that means we
    // didn't count the final char in the sequence.  Luckily, the final char in
    // the base sequence is also the final char in the end sequence, so we can
    // just increment that count by one now.
    *counts.entry(self.state[self.state.len() - 1]).or_insert(0) += 1;
    let (mut min, mut max) = (0u64, 0u64);
    for (_k, v) in counts {
      if v < min || min == 0 {
        min = v;
      }
      if v > max {
        max = v;
      }
    }
    max - min
  }
}

pub fn part_one(input: &str) -> u64 {
  let mut polymerizer = Polymerizer::new(input);
  polymerizer.run(10);
  polymerizer.get_min_max_diff()
}

pub fn part_two(input: &str) -> u64 {
  let mut polymerizer = Polymerizer::new(input);
  polymerizer.run(40);
  polymerizer.get_min_max_diff()
}

#[cfg(test)]
mod day14_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"), 1588);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"), 2188189693529);
  }
}
