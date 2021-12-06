use aoc_helpers::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

const YEAR: u32 = 2020;
const DAY: u32 = 16;

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

#[derive(Debug)]
pub struct Rule {
  name: Rc<String>,
  range1: (u64, u64),
  range2: (u64, u64),
  possible_indices: HashSet<usize>,
  true_index: Option<usize>,
}

type Rules = HashMap<Rc<String>, Rule>;

pub fn parse_rules<'a, I>(lines: &mut I) -> Rules
where
  I: Iterator<Item = &'a str>,
{
  let mut rules: Rules = HashMap::new();
  while let Some(line) = lines.next() {
    if line == "" {
      return rules;
    }
    let mut parts = line.split(": ");
    let key = Rc::new(parts.next().unwrap().to_owned());
    let mut ranges = parts.next().unwrap().split(" or ");
    let mut range1 = ranges.next().unwrap().split('-');
    let mut range2 = ranges.next().unwrap().split('-');
    let rule = Rule {
      name: key.clone(),
      range1: (
        range1.next().unwrap().parse::<u64>().unwrap(),
        range1.next().unwrap().parse::<u64>().unwrap(),
      ),
      range2: (
        range2.next().unwrap().parse::<u64>().unwrap(),
        range2.next().unwrap().parse::<u64>().unwrap(),
      ),
      possible_indices: HashSet::new(),
      true_index: None,
    };
    rules.insert(key, rule);
  }
  rules
}

pub fn parse_ticket(line: &str) -> Vec<u64> {
  line.split(',').map(|s| s.parse::<u64>().unwrap()).collect()
}

pub fn parse_their_ticket<'a, I>(lines: &mut I) -> Vec<Vec<u64>>
where
  I: Iterator<Item = &'a str>,
{
  lines.map(|s| parse_ticket(s)).collect()
}

pub fn validate_ticket(ticket: &Vec<u64>, rules: &Rules) -> Option<u64> {
  let mut invalid = None;
  for num in ticket {
    let mut valid = false;
    for rule in rules.values() {
      if (num >= &rule.range1.0 && num <= &rule.range1.1)
        || (num >= &rule.range2.0 && num <= &rule.range2.1)
      {
        valid = true;
        break;
      }
    }
    if !valid {
      if invalid == None {
        invalid = Some(0);
      }
      invalid = Some(num + invalid.unwrap());
    }
  }
  invalid
}

pub fn part_one(input: &str) -> u64 {
  let mut lines = input.lines();
  let rules = parse_rules(&mut lines);
  lines.next(); // Consume the 'your ticket:' header
  let _your_ticket = parse_ticket(lines.next().unwrap());
  lines.next(); // Consume the blank line
  lines.next(); // Consume the 'nearby tickets' header
  let their_tickets = parse_their_ticket(&mut lines);
  their_tickets
    .iter()
    .map(|t| {
      if let Some(inv) = validate_ticket(t, &rules) {
        inv
      } else {
        0
      }
    })
    .sum()
}

pub fn part_two(input: &str) -> u64 {
  let mut lines = input.lines();
  let mut rules = parse_rules(&mut lines);
  lines.next(); // Consume the 'your ticket:' header
  let your_ticket = parse_ticket(lines.next().unwrap());
  lines.next(); // Consume the blank line
  lines.next(); // Consume the 'nearby tickets' header
  let their_tickets = parse_their_ticket(&mut lines);
  let mut all_tickets: Vec<&Vec<u64>> = their_tickets
    .iter()
    .filter(|t| validate_ticket(t, &rules) == None)
    .collect();
  all_tickets.push(&your_ticket);

  // Narrow down each rule's possible_indices to only the indices for which
  // every ticket has a valid value in that slot.
  let field_count = your_ticket.len();
  let mut resolved: HashSet<usize> = HashSet::new();
  for (_, rule) in rules.iter_mut() {
    (*rule).possible_indices = (0..field_count).collect();
    for i in 0..field_count {
      for t in &all_tickets {
        let num = t[i];
        if (num >= rule.range1.0 && num <= rule.range1.1)
          || (num >= rule.range2.0 && num <= rule.range2.1)
        {
          continue;
        }
        rule.possible_indices.remove(&i);
        break;
      }
    }
    if rule.possible_indices.len() == 1 {
      let true_index = rule.possible_indices.iter().next().unwrap();
      (*rule).true_index = Some(*true_index);
      resolved.insert(*true_index);
    }
  }

  // Now we need to keep iterating over the rules to see if there are any
  // indices that are only valid for a single rule.  Keep repeating this loop
  // until every rule has a true_index.  We can try to calculate the product
  // along the way.
  let mut something_changed = true;
  let mut product = 1;
  let mut product_count = 0_u64;
  while resolved.len() < field_count && something_changed {
    something_changed = false;
    let mut reducer: HashMap<usize, HashSet<Rc<String>>> = HashMap::new();
    for (name, rule) in &rules {
      if let Some(_) = rule.true_index {
        continue;
      }
      for i in &rule.possible_indices {
        let e = reducer.entry(*i).or_insert(HashSet::new());
        e.insert(name.clone());
      }
    }
    for (i, names) in reducer {
      if names.len() == 1 {
        let name = names.iter().next().unwrap();
        rules.get_mut(name).unwrap().true_index = Some(i);
        resolved.insert(i);
        something_changed = true;
        if name.starts_with("departure") {
          product_count += 1;
          product *= your_ticket[i];
        }
      }
    }
  }

  // If we weren't able to figure out every field, panic.  If we at least
  // managed to get all 6 'departed' rules though, print that out still.
  if resolved.len() < field_count {
    if product_count == 6 {
      println!("managed to find the answer: {}", product);
    }
    panic!("didn't end up finding a true index for every rule");
  }

  product
}

#[cfg(test)]
mod day16_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
      ),
      71
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
