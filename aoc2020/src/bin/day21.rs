use aoc_helpers::*;
use itertools::join;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2020
    }
    fn day(&self) -> u32 {
        21
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

type AllsToIngs = HashMap<String, HashSet<String>>;
type IngsToAlls = HashMap<String, HashSet<String>>;
type IngCounts = HashMap<String, usize>;

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
                *e = e.intersection(&ings_line).cloned().collect();
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
    // mapping.  If an allergen is on a line, its ingredient will always be
    // present on that line as well.  However, the reverse isn't necessarily true.
    // As a result, we rely on the allergens -> ingredients mapping as a source of
    // truth (trying to build the reverse mapping iteratively above would end up
    // being inaccurate).
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

pub fn reduce(alls_to_ings: AllsToIngs) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut resolved_ings: HashSet<String> = HashSet::new();
    let mut deque: VecDeque<(String, HashSet<String>)> = alls_to_ings
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    while let Some((a, ings)) = deque.pop_front() {
        if ings.len() == 1 {
            let ing = ings.iter().next().unwrap();
            resolved_ings.insert(ing.to_string());
            out.push((a, ing.to_string()));
            continue;
        }
        let mut new_ings: HashSet<String> = HashSet::new();
        ings.iter()
            .filter(|ing| !resolved_ings.contains(&ing.to_string()))
            .for_each(|ing| {
                new_ings.insert(ing.to_string());
            });
        deque.push_back((a, new_ings));
    }
    out
}

pub fn part_two(input: &str) -> String {
    let (_ings_to_alls, alls_to_ings, _ing_counts) = parse_ingredients(input);
    let mut mappings = reduce(alls_to_ings);
    mappings.sort_by(|a, b| a.0.cmp(&b.0));
    join(mappings.iter().map(|(_, ing)| ing), ",")
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
        assert_eq!(
            part_two(
                "mxmnhmsxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmnhmsxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc fvjkl mxmnhmsxvkd (contains soy)
sqjhc mxmnhmsxvkd sbzzf (contains fish)"
            ),
            "mxmnhmsxvkd,sqjhc,fvjkl"
        );
    }
}
