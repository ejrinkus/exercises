use aoc_helpers::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2015
    }
    fn day(&self) -> u32 {
        13
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

lazy_static! {
  static ref RE: Regex = Regex::new(r"([[:alpha:]]+) would ([[:alpha:]]+) (\d+) happiness units by sitting next to ([[:alpha:]]+)").unwrap();
}

fn parse_line(line: &str) -> (&str, &str, i64) {
    let caps = RE.captures(line).unwrap();
    let person1 = caps.get(1).unwrap().as_str();
    let person2 = caps.get(4).unwrap().as_str();
    let gain_lose = caps.get(2).unwrap().as_str();
    let mut units = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
    if gain_lose == "lose" {
        units *= -1;
    }
    (person1, person2, units)
}

pub fn part_one(input: &str) -> i64 {
    let mut people: HashSet<String> = HashSet::new();
    let mut combos: Vec<(String, String, i64)> = Vec::new();
    for line in input.lines() {
        let combo = parse_line(line);
        people.insert(combo.0.to_string());
        people.insert(combo.1.to_string());
        let mut exists = false;
        for c in combos.iter_mut() {
            if combo.0 == c.1 && combo.1 == c.0 {
                // This combo already exists, just update it's happiness value.
                c.2 += combo.2;
                exists = true;
            }
        }
        if !exists {
            combos.push((combo.0.to_string(), combo.1.to_string(), combo.2));
        }
    }
    combos.sort_unstable_by_key(|(_p1, _p2, u)| *u);
    let mut people_dupes: HashSet<String> = HashSet::new();
    let mut seating: Vec<(String, String, i64)> = Vec::new();
    // Put the highest-happiness couple next to each other, then greedily add more
    // people to the seating arrangement.
    let mut combo = combos.pop().unwrap();
    people.remove(&combo.0);
    people.remove(&combo.1);
    seating.push(combo);
    while !people.is_empty() {
        let left_combo = &seating[0];
        let right_combo = &seating[seating.len() - 1];
        for i in (0..combos.len()).rev() {
            if people.contains(&combos[i].0) && people.contains(&combos[i].1) {
                // Neither person is seated, therefore neither person represents the
                // head or tail of the seating arrangement; move on.
                continue;
            }
            if !people.contains(&combos[i].0) && !people.contains(&combos[i].1) {
                // Both people in this combo are already seated; move on.
                continue;
            }
            // If one person is seated and one is not, then this combo might be able
            // to link to one of the ends of the arrangement.
            if &combos[i].0 == &left_combo.0
                || &combos[i].0 == &left_combo.1
                || &combos[i].1 == &left_combo.0
                || &combos[i].1 == &left_combo.1
            {
                // This combo matches one of the people on the left side of the
                // arrangmenet; move this combo to that side.
                combo = combos.remove(i);
                if people.contains(&combo.0) {
                    people.remove(&combo.0);
                } else {
                    people_dupes.insert(combo.0.to_string());
                }
                if people.contains(&combo.1) {
                    people.remove(&combo.1);
                } else {
                    people_dupes.insert(combo.1.to_string());
                }
                seating.insert(0, combo);
                break;
            }
            if &combos[i].0 == &right_combo.0
                || &combos[i].0 == &right_combo.1
                || &combos[i].1 == &right_combo.0
                || &combos[i].1 == &right_combo.1
            {
                // This combo matches one of the people on the right side of the
                // arrangmenet; move this combo to that side.
                combo = combos.remove(i);
                if people.contains(&combo.0) {
                    people.remove(&combo.0);
                } else {
                    people_dupes.insert(combo.0.to_string());
                }
                if people.contains(&combo.1) {
                    people.remove(&combo.1);
                } else {
                    people_dupes.insert(combo.1.to_string());
                }
                seating.push(combo);
                break;
            }
        }
    }

    // Once we're done, the seating list will still be missing the final link
    // between the two ends of the arrangement.  Find that link and add it to the
    // list.
    for i in 0..combos.len() {
        if people_dupes.contains(&combos[i].0) || people_dupes.contains(&combos[i].1) {
            continue;
        }
        combo = combos.remove(i);
        seating.push(combo);
        break;
    }
    seating.iter().fold(0, |acc, (_p1, _p2, u)| acc + u)
}

pub fn part_two(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
            ),
            330
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two(""), 0);
    }

    #[test]
    fn parse_line_test() {
        assert_eq!(
            parse_line("Alice would gain 54 happiness units by sitting next to Bob."),
            ("Alice", "Bob", 54)
        );
        assert_eq!(
            parse_line("Alice would lose 79 happiness units by sitting next to Carol."),
            ("Alice", "Carol", -79)
        );
        assert_eq!(
            parse_line("Alice would lose 2 happiness units by sitting next to David."),
            ("Alice", "David", -2)
        );
        assert_eq!(
            parse_line("Bob would gain 83 happiness units by sitting next to Alice."),
            ("Bob", "Alice", 83)
        );
        assert_eq!(
            parse_line("Bob would lose 7 happiness units by sitting next to Carol."),
            ("Bob", "Carol", -7)
        );
        assert_eq!(
            parse_line("Bob would lose 63 happiness units by sitting next to David."),
            ("Bob", "David", -63)
        );
        assert_eq!(
            parse_line("Carol would lose 62 happiness units by sitting next to Alice."),
            ("Carol", "Alice", -62)
        );
        assert_eq!(
            parse_line("Carol would gain 60 happiness units by sitting next to Bob."),
            ("Carol", "Bob", 60)
        );
        assert_eq!(
            parse_line("Carol would gain 55 happiness units by sitting next to David."),
            ("Carol", "David", 55)
        );
        assert_eq!(
            parse_line("David would gain 46 happiness units by sitting next to Alice."),
            ("David", "Alice", 46)
        );
        assert_eq!(
            parse_line("David would lose 7 happiness units by sitting next to Bob."),
            ("David", "Bob", -7)
        );
        assert_eq!(
            parse_line("David would gain 41 happiness units by sitting next to Carol."),
            ("David", "Carol", 41)
        );
    }
}
