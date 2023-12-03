use aoc_helpers::*;
use std::collections::HashMap;
use std::collections::HashSet;

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
        7
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

type ColorMap = HashMap<String, HashSet<(String, u32)>>;

pub fn build_color_map(input: &str) -> ColorMap {
    let mut color_map: ColorMap = HashMap::new();
    for line in input.lines() {
        let mut pieces = line.split(" bags contain ");
        let color = pieces.next().unwrap();
        if line.contains("no other bags") {
            color_map.insert(color.to_string(), HashSet::new());
            continue;
        }
        let rest = pieces
            .next()
            .unwrap()
            .replace(" bags", "")
            .replace(" bag", "");
        let rest = rest.trim_end_matches('.');
        let rest = rest.trim();
        let mut inner_colors: HashSet<(String, u32)> = HashSet::new();
        for count_color in rest.split(", ") {
            let mut pieces = count_color.splitn(2, ' ');
            let count = pieces.next().unwrap().parse::<u32>().unwrap();
            let inner_c = pieces.next().unwrap().to_string();
            inner_colors.insert((inner_c, count));
        }
        color_map.insert(color.to_string(), inner_colors);
    }
    color_map
}

pub fn find_color(start: &str, color: &str, color_map: &ColorMap) -> bool {
    let maybe_set = color_map.get(start);
    if maybe_set.is_none() {
        return false;
    }
    let set = maybe_set.unwrap();
    for (c, _count) in set {
        if c == color {
            return true;
        }
    }
    for (c, _count) in set.iter() {
        if find_color(c, color, color_map) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> u32 {
    let color_map = build_color_map(input);

    let mut count = 0;
    for (k, _v) in &color_map {
        if find_color(&k, "shiny gold", &color_map) {
            count += 1;
        }
    }

    count
}

pub fn get_contents(color: &str, color_map: &ColorMap) -> u32 {
    let mut total = 0;
    for (inner_color, count) in color_map.get(color).unwrap() {
        total += (1 + get_contents(inner_color, color_map)) * count;
    }
    total
}

pub fn part_two(input: &str) -> u32 {
    let color_map = build_color_map(input);
    get_contents("shiny gold", &color_map)
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            ),
            4
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            ),
            32
        );
    }
}
