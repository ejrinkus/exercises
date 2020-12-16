use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Rule {
  name: Rc<String>,
  range1: (u32, u32),
  range2: (u32, u32),
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
        range1.next().unwrap().parse::<u32>().unwrap(),
        range1.next().unwrap().parse::<u32>().unwrap(),
      ),
      range2: (
        range2.next().unwrap().parse::<u32>().unwrap(),
        range2.next().unwrap().parse::<u32>().unwrap(),
      ),
    };
    rules.insert(key, rule);
  }
  rules
}

pub fn parse_ticket(line: &str) -> Vec<u32> {
  line.split(',').map(|s| s.parse::<u32>().unwrap()).collect()
}

pub fn parse_their_ticket<'a, I>(lines: &mut I) -> Vec<Vec<u32>>
where
  I: Iterator<Item = &'a str>,
{
  lines.map(|s| parse_ticket(s)).collect()
}

pub fn validate_ticket(ticket: &Vec<u32>, rules: &Rules) -> u32 {
  let mut invalid = 0;
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
      invalid += num;
    }
  }
  invalid
}

pub fn part_one(input: &str) -> u32 {
  let mut lines = input.lines();
  let rules = parse_rules(&mut lines);
  lines.next(); // Consume the 'your ticket:' header
  let _your_ticket = parse_ticket(lines.next().unwrap());
  lines.next(); // Consume the blank line
  lines.next(); // Consume the 'nearby tickets' header
  let their_tickets = parse_their_ticket(&mut lines);
  their_tickets
    .iter()
    .map(|t| validate_ticket(t, &rules))
    .sum()
}

pub fn part_two(input: &str) -> i64 {
  println!("{}", input);
  0
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
