use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        12
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

struct Caves {
    edges: HashMap<String, Vec<String>>,
    paths: Vec<Vec<String>>,
    part_two: bool,
}

impl Caves {
    pub fn new(input: &str, part_two: bool) -> Caves {
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        for line in input.lines() {
            let mut edge = line.split('-');
            let cave1 = edge.next().unwrap();
            let cave2 = edge.next().unwrap();
            edges
                .entry(cave1.to_string())
                .or_insert(Vec::new())
                .push(cave2.to_string());
            edges
                .entry(cave2.to_string())
                .or_insert(Vec::new())
                .push(cave1.to_string());
        }
        Caves {
            edges: edges,
            paths: Vec::new(),
            part_two: part_two,
        }
    }

    fn is_small(cave: &str) -> bool {
        cave.chars().next().unwrap().is_ascii_lowercase()
    }

    fn is_cave_valid_for_path(&self, cave: &str, path: &Vec<String>) -> bool {
        if !Caves::is_small(cave) {
            return true;
        }
        if cave == "start" {
            return false;
        }
        let mut visited: HashSet<String> = HashSet::new();
        let mut repeats = 0;
        for c in path {
            if !Caves::is_small(c) {
                continue;
            }
            if !visited.insert(c.to_string()) {
                repeats += 1;
            }
            if c == cave {
                repeats += 1;
            }
            if (self.part_two && repeats > 1) || (!self.part_two && repeats > 0) {
                return false;
            }
        }
        true
    }

    pub fn build_paths(&mut self) {
        let mut potentials: Vec<Vec<String>> = Vec::new();
        potentials.push(vec!["start".to_string()]);
        while !potentials.is_empty() {
            let base_path = potentials.pop().unwrap();
            for cave in &self.edges[&base_path[base_path.len() - 1]] {
                if cave == "end" {
                    let mut clone = base_path.clone();
                    clone.push(cave.to_string());
                    self.paths.push(clone);
                } else if self.is_cave_valid_for_path(cave, &base_path) {
                    let mut clone = base_path.clone();
                    clone.push(cave.to_string());
                    potentials.push(clone);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut caves = Caves::new(input, false);
    caves.build_paths();
    caves.paths.len()
}

pub fn part_two(input: &str) -> usize {
    let mut caves = Caves::new(input, true);
    caves.build_paths();
    // for path in caves.paths {
    //   let path_str = path.join(",");
    //   println!("{}", path_str);
    // }
    caves.paths.len()
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ),
            10
        );
        assert_eq!(
            part_one(
                "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            ),
            19
        );
        assert_eq!(
            part_one(
                "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            ),
            226
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ),
            36
        );
        assert_eq!(
            part_two(
                "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            ),
            103
        );
        assert_eq!(
            part_two(
                "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            ),
            3509
        );
    }
}
