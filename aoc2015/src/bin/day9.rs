use aoc_helpers::*;
use std::collections::HashMap;
use std::iter::FromIterator;

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
        9
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

type Edges = HashMap<String, HashMap<String, usize>>;

pub fn build_graph(input: &str) -> Edges {
    let mut edges: Edges = HashMap::new();
    for line in input.lines() {
        let pieces: Vec<&str> = line.split(' ').collect();
        if pieces.len() != 5 {
            panic!("Invalid line: {}", line);
        }
        let start = pieces[0];
        let end = pieces[2];
        let dist = pieces[4].parse::<usize>().unwrap();
        let city_edges = edges.entry(start.to_string()).or_insert(HashMap::new());
        city_edges.insert(end.to_string(), dist);
        let city_edges = edges.entry(end.to_string()).or_insert(HashMap::new());
        city_edges.insert(start.to_string(), dist);
    }
    edges
}

pub fn find_shortest(start: &str, cities: Vec<String>, edges: &Edges) -> usize {
    if cities.len() == 0 {
        return 0;
    }
    if cities.len() == 1 {
        return *edges.get(start).unwrap().get(&cities[0]).unwrap();
    }

    let mut shortest = usize::MAX;
    for (i, city) in cities.iter().enumerate() {
        let mut new_cities = cities.clone();
        new_cities.remove(i);
        let mut dist = find_shortest(&city, new_cities, edges);
        if start != "" {
            dist += *edges.get(start).unwrap().get(city).unwrap();
        }
        if dist < shortest {
            shortest = dist;
        }
    }
    shortest
}

pub fn find_longest(start: &str, cities: Vec<String>, edges: &Edges) -> usize {
    if cities.len() == 0 {
        return 0;
    }
    if cities.len() == 1 {
        return *edges.get(start).unwrap().get(&cities[0]).unwrap();
    }

    let mut longest = 0;
    for (i, city) in cities.iter().enumerate() {
        let mut new_cities = cities.clone();
        new_cities.remove(i);
        let mut dist = find_longest(&city, new_cities, edges);
        if start != "" {
            dist += *edges.get(start).unwrap().get(city).unwrap();
        }
        if dist > longest {
            longest = dist;
        }
    }
    longest
}

pub fn part_one(input: &str) -> i64 {
    // Build the graph.
    let edges: Edges = build_graph(input);

    // Brute force traveling salesman solution.
    let cities = Vec::from_iter(edges.keys().map(|k| k.to_string()));
    find_shortest("", cities, &edges) as i64
}

pub fn part_two(input: &str) -> i64 {
    // Build the graph.
    let edges: Edges = build_graph(input);

    // Brute force traveling salesman solution.
    let cities = Vec::from_iter(edges.keys().map(|k| k.to_string()));
    find_longest("", cities, &edges) as i64
}

#[cfg(test)]
mod day9_tests {
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
