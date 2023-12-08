use aoc_helpers::math::lcm;
use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::collections::HashMap;
use std::str::Lines;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        8
    }

    fn part_one(&self, input: &str) -> String {
        let mut iter = input.lines();
        let directions = parse_directions(iter.next().expect("Missing directions line"));
        iter.next(); // Skip blank line

        let node_map = parse_nodes(iter);

        let mut steps = 0;
        let mut node = "AAA";
        let mut i = 0;
        while let Some((left, right)) = node_map.get(node) {
            if directions[i] == 'L' {
                node = left;
            } else {
                node = right;
            }
            steps += 1;
            i += 1;
            if i >= directions.len() {
                i = 0;
            }
            if node == "ZZZ" {
                break;
            }
        }
        if node != "ZZZ" {
            panic!("Uh oh, didn't find a path from AAA to ZZZ");
        }
        steps.to_string()
    }

    // This solution relies on two assumptions that _are actually true_ for the input we're given:
    // 1. It takes n steps for a start node to reach a corresponding end node, and there's a closed
    //    loop of length m which will return to that same end node without hitting any other potential
    //    end nodes.
    // 2. n is equal to m.
    //
    // Because these assumptions are true, each starting node effectively begins a loop where every
    // n steps leads back to the end node that it maps to.  So we can simply apply LCM sequentially
    // to all those loop sizes to calculate the minimum number of steps to reach our destination for
    // all starting points.
    fn part_two(&self, input: &str) -> String {
        let mut iter = input.lines();
        let directions = parse_directions(iter.next().expect("Missing directions line"));
        iter.next(); // Skip blank line

        let node_map = parse_nodes(iter);

        let mut nodes: Vec<&str> = node_map
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| k as &str)
            .collect();
        let mut loops: Vec<u64> = nodes.iter().map(|_| 0u64).collect();

        let mut steps = 0;
        let mut i = 0;
        let mut closed_loops = 0;
        while closed_loops < loops.len() {
            steps += 1;
            for j in 0..nodes.len() {
                if loops[j] != 0 {
                    continue;
                }
                let node = nodes[j];
                let (left, right) = node_map.get(node).expect("didn't find key node");
                if directions[i] == 'L' {
                    nodes[j] = left as &str;
                } else {
                    nodes[j] = right as &str;
                }
                if nodes[j].ends_with('Z') {
                    loops[j] = steps;
                    closed_loops += 1;
                }
            }
            i += 1;
            if i >= directions.len() {
                i = 0;
            }
        }

        steps = loops[0];
        for l in loops.iter().skip(1) {
            steps = lcm(steps, *l);
        }

        steps.to_string()
    }
}

fn parse_directions(s: &str) -> Vec<char> {
    let mut rem = s;
    let mut dir: char;
    let mut ret = Vec::<char>::new();
    while rem != "" {
        (rem, dir) = take_char(rem).expect("Missing direction character");
        ret.push(dir);
    }
    ret
}

fn parse_nodes(mut iter: Lines<'_>) -> HashMap<String, (String, String)> {
    let mut rem: &str;
    let mut start: &str;
    let mut dest1: &str;
    let mut dest2: &str;
    let mut node_map = HashMap::<String, (String, String)>::new();
    while let Some(line) = iter.next() {
        (rem, start) = take_n(line, 3).expect("Couldn't match on the start node");
        (rem, _) = take_tag(rem, " = (").expect("Couldn't trim the ' = (' tag");
        (rem, dest1) = take_n(rem, 3).expect("Couldn't match on dest1");
        (rem, _) = take_tag(rem, ", ").expect("Couldn't trim the ', ' tag");
        (rem, dest2) = take_n(rem, 3).expect("Couldn't match on dest2");
        node_map.insert(start.to_string(), (dest1.to_string(), dest2.to_string()));
    }
    node_map
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            "2"
        );
        assert_eq!(
            solution.part_one(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            "6"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            "6"
        );
    }
}
