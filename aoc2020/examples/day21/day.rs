use std::collections::{HashMap, HashSet};

type AllsToIngs = HashMap<String, HashSet<String>>;
type IngsToAlls = HashMap<String, HashSet<String>>;

pub fn intersect(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String> {
  let mut out: HashSet<String> = HashSet::new();
  for s in a {
    if b.contains(s) {
      out.insert(s.to_string());
    }
  }
  out
}

pub fn parse_line(line: &str) -> (HashSet<String>, HashSet<String>) {
  let mut ings: HashSet<String> = HashSet::new();
  let mut alls: HashSet<String> = HashSet::new();
  let mut found_contains = false;
  for p in line.split(' ') {
    // Until we find "(contains", we're dealing with ingredients.
    if p == "(contains" {
      found_contains = true;
      continue;
    } else if !found_contains {
      // Store in the set for this line.
      ings.insert(p.to_string());
      continue;
    }
    // Once we've found "(contains", we're dealing with allergens.
    // Strip punctuation, and store in the set.
    if p.ends_with(')') {
      alls.insert(p.strip_suffix(")").unwrap().to_string());
    } else if p.ends_with(',') {
      alls.insert(p.strip_suffix(",").unwrap().to_string());
    } else {
      alls.insert(p.to_string());
    }
  }
  (ings, alls)
}

pub fn parse_ingredients(input: &str) -> (IngsToAlls, AllsToIngs) {
  let mut ings_to_alls: IngsToAlls = IngsToAlls::new();
  let mut alls_to_ings: AllsToIngs = AllsToIngs::new();
  for line in input.lines() {
    let (mut ings_line, mut alls_line) = parse_line(line);
    // Map ingredients to possible allergens.  If there's already a set of possible
    // allergens for an ingredient, then intersect those sets.
    for i in &ings_line {
      let e = ings_to_alls.entry(i.clone()).or_insert(HashSet::new());
      if e.is_empty() {
        *e = alls_line.clone();
      } else {
        *e = intersect(e, &alls_line);
      }
    }
    // Same as above, but for allergens to ingredients.  Note that a complete solution
    // would result in only one ingredient in each of these mappings.
    for a in &alls_line {
      let e = alls_to_ings.entry(a.clone()).or_insert(HashSet::new());
      if e.is_empty() {
        *e = ings_line.clone();
      } else {
        *e = intersect(e, &ings_line);
      }
    }
  }
  (ings_to_alls, alls_to_ings)
}

pub fn part_one(input: &str) -> usize {
  let (ings_to_alls, alls_to_ings) = parse_ingredients(input);
  println!("ings_to_alls: {:?}", ings_to_alls);
  println!("alls_to_ings: {:?}", alls_to_ings);
  let mut sum = 0;
  for (ing, alls) in ings_to_alls {
    if alls.len() != 0 {
      continue;
    }
    sum += input.match_indices(&ing).count();
  }
  sum
}

pub fn part_two(input: &str) -> i64 {
  println!("{}", input);
  0
}

#[cfg(test)]
mod day21_tests {
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
