use std::collections::{HashMap, HashSet};

type AllsToIngs = HashMap<String, HashSet<String>>;
type IngsToAlls = HashMap<String, HashSet<String>>;
type IngCounts = HashMap<String, usize>;

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

pub fn parse_ingredients(input: &str) -> (IngsToAlls, AllsToIngs, IngCounts) {
  let mut alls_to_ings: AllsToIngs = AllsToIngs::new();
  let mut ings_to_alls: IngsToAlls = IngsToAlls::new();
  let mut ing_counts: IngCounts = IngCounts::new();
  for line in input.lines() {
    let (ings_line, alls_line) = parse_line(line);
    // Each allergen on the line is contained within exactly one ingredient on
    // the line.  Since each allergen can only be found in exactly one
    // ingredient, we want to keep intersecting these sets to narrow down the
    // possibilities.
    for a in &alls_line {
      let e = alls_to_ings.entry(a.clone()).or_insert(HashSet::new());
      if e.is_empty() {
        *e = ings_line.clone();
      } else {
        *e = intersect(e, &ings_line);
      }
    }
    // Keep track of every ingredient we've found so far
    for ing in &ings_line {
      ings_to_alls.entry((*ing).clone()).or_insert(HashSet::new());
      let count = ing_counts.entry((*ing).clone()).or_insert(0);
      *count += 1;
    }
  }
  // Once we've built alls_to_ings, we want to flip it to have the reverse
  // mapping.  Since an ingredient can contain multiple allergens, and because
  // all allergens don't always appear on a line, we can't use the same
  // iterative intersection approach that we used with alls_to_ings, so we do it
  // here.
  for (a, ings) in &alls_to_ings {
    for ing in ings {
      ings_to_alls
        .entry((*ing).clone())
        .or_insert(HashSet::new())
        .insert(a.clone());
    }
  }
  (ings_to_alls, alls_to_ings, ing_counts)
}

pub fn part_one(input: &str) -> usize {
  let (ings_to_alls, _alls_to_ings, ing_counts) = parse_ingredients(input);
  let mut sum = 0;
  for (ing, alls) in ings_to_alls {
    if alls.len() != 0 {
      continue;
    }
    sum += ing_counts.get(&ing).unwrap();
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
    assert_eq!(
      part_one(
        "mxmnhmsxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmnhmsxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc fvjkl mxmnhmsxvkd (contains soy)
sqjhc mxmnhmsxvkd sbzzf (contains fish)"
      ),
      5
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
