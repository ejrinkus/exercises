use aoc_helpers::*;
use std::collections::{HashSet, VecDeque};

const YEAR: u32 = 2020;
const DAY: u32 = 22;

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

type Deck = VecDeque<u32>;

pub fn build_decks(input: &str) -> (Deck, Deck) {
  let mut p1: Deck = Deck::new();
  let mut p2: Deck = Deck::new();
  let mut player = 0;
  for line in input.lines() {
    if line == "Player 1:" {
      player = 1;
      continue;
    } else if line == "Player 2:" {
      player = 2;
      continue;
    } else if line == "" {
      continue;
    }
    if player == 1 {
      p1.push_back(line.parse::<u32>().unwrap());
    } else if player == 2 {
      p2.push_back(line.parse::<u32>().unwrap());
    }
  }
  (p1, p2)
}

pub fn get_score(deck: &Deck) -> u32 {
  let size = deck.len();
  deck
    .iter()
    .enumerate()
    .fold(0_u32, |acc, (i, v)| acc + (v * (size - i) as u32))
}

// Returns winner and their score.
pub fn run_combat(p1: &mut Deck, p2: &mut Deck) -> (usize, u32) {
  while p1.len() > 0 && p2.len() > 0 {
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    if c1 > c2 {
      p1.push_back(c1);
      p1.push_back(c2);
    } else if c2 > c1 {
      p2.push_back(c2);
      p2.push_back(c1);
    }
  }
  if p1.len() > 0 {
    (1, get_score(p1))
  } else {
    (2, get_score(p2))
  }
}

// Returns winner and their score.
pub fn run_recursive_combat(p1: &mut Deck, p2: &mut Deck) -> (usize, u32) {
  let mut states: HashSet<(u32, u32)> = HashSet::new();
  while p1.len() > 0 && p2.len() > 0 {
    // Check the game state.
    let state = (get_score(p1), get_score(p2));
    if states.contains(&state) {
      return (1, get_score(p1));
    }
    states.insert(state);
    // Draw cards
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    // Compare drawn cards against deck sizes
    if c1 as usize <= p1.len() && c2 as usize <= p2.len() {
      let mut p1_cpy: VecDeque<u32> = p1.iter().take(c1 as usize).copied().collect();
      let mut p2_cpy: VecDeque<u32> = p2.iter().take(c2 as usize).copied().collect();
      let result = run_recursive_combat(&mut p1_cpy, &mut p2_cpy);
      if result.0 == 1 {
        p1.push_back(c1);
        p1.push_back(c2);
      } else if result.0 == 2 {
        p2.push_back(c2);
        p2.push_back(c1);
      }
      continue;
    }
    // Play ordinary round
    if c1 > c2 {
      p1.push_back(c1);
      p1.push_back(c2);
    } else if c2 > c1 {
      p2.push_back(c2);
      p2.push_back(c1);
    }
  }
  if p1.len() > 0 {
    (1, get_score(p1))
  } else {
    (2, get_score(p2))
  }
}

pub fn part_one(input: &str) -> u32 {
  let (mut p1, mut p2) = build_decks(input);
  run_combat(&mut p1, &mut p2).1
}

pub fn part_two(input: &str) -> u32 {
  let (mut p1, mut p2) = build_decks(input);
  run_recursive_combat(&mut p1, &mut p2).1
}

#[cfg(test)]
mod day22_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
      ),
      306
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
      ),
      291
    );
  }
}
