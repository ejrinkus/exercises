use aoc_helpers::*;
use std::collections::HashMap;
use std::iter::FromIterator;

const YEAR: u32 = 2015;
const DAY: u32 = 9;

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
