use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord3 {
  x: i64,
  y: i64,
  z: i64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord4 {
  x: i64,
  y: i64,
  z: i64,
  w: i64,
}

type Dimension3 = HashMap<Coord3, bool>;
type Dimension4 = HashMap<Coord4, bool>;

pub fn parse_input(input: &str) -> Dimension3 {
  let mut dimension: Dimension3 = Dimension3::new();
  for (y, line) in input.lines().enumerate() {
    for (x, c) in line.char_indices() {
      let coord = Coord3 {
        x: x as i64,
        y: y as i64,
        z: 0,
      };
      match c {
        '#' => {
          dimension.insert(coord, true);
        }
        '.' => {
          dimension.insert(coord, false);
        }
        _ => {
          panic!("unexpected input: {}", c);
        }
      }
    }
  }
  dimension
}

pub fn parse_input2(input: &str) -> Dimension4 {
  let mut dimension: Dimension4 = Dimension4::new();
  for (y, line) in input.lines().enumerate() {
    for (x, c) in line.char_indices() {
      let coord = Coord4 {
        x: x as i64,
        y: y as i64,
        z: 0,
        w: 0,
      };
      match c {
        '#' => {
          dimension.insert(coord, true);
        }
        '.' => {
          dimension.insert(coord, false);
        }
        _ => {
          panic!("unexpected input: {}", c);
        }
      }
    }
  }
  dimension
}

pub fn get_neighbors(start: &Coord3) -> Vec<Coord3> {
  let mut neighbors: Vec<Coord3> = Vec::new();
  for x_off in -1..2 {
    for y_off in -1..2 {
      for z_off in -1..2 {
        if x_off == 0 && y_off == 0 && z_off == 0 {
          continue;
        }
        neighbors.push(Coord3 {
          x: start.x + x_off,
          y: start.y + y_off,
          z: start.z + z_off,
        });
      }
    }
  }
  if neighbors.len() != 26 {
    panic!("only got {} neighbors");
  }
  neighbors
}

pub fn get_neighbors2(start: &Coord4) -> Vec<Coord4> {
  let mut neighbors: Vec<Coord4> = Vec::new();
  for x_off in -1..2 {
    for y_off in -1..2 {
      for z_off in -1..2 {
        for w_off in -1..2 {
          if x_off == 0 && y_off == 0 && z_off == 0 && w_off == 0 {
            continue;
          }
          neighbors.push(Coord4 {
            x: start.x + x_off,
            y: start.y + y_off,
            z: start.z + z_off,
            w: start.w + w_off,
          });
        }
      }
    }
  }
  if neighbors.len() != 80 {
    panic!("only got {} neighbors");
  }
  neighbors
}

pub fn part_one(input: &str) -> i64 {
  let mut dimension = parse_input(input);
  let mut count = 0_i64;
  for i in 0..6 {
    // Since we're using a map, we need to get every active cube in the dimension
    // and insert inactive cubes in every adjacent position that doesn't already
    // have a cube.
    let mut new_neighbors: HashSet<Coord3> = HashSet::new();
    for (k, v) in &dimension {
      if !v {
        continue;
      }
      for n in get_neighbors(&k) {
        new_neighbors.insert(n);
      }
    }
    for k in new_neighbors {
      dimension.entry(k).or_default();
    }

    // Clone the existing dimension so that we can make a replacement.
    let old_dimension = dimension.clone();

    // Build the new dimension from the old one.
    dimension = Dimension3::new();
    for (k, v) in &old_dimension {
      let neighbors = get_neighbors(&k);
      let mut active = 0;
      for c in neighbors {
        if let Some(n) = old_dimension.get(&c) {
          if *n == true {
            active += 1;
          }
        }
      }
      if active == 3 || (active == 2 && *v == true) {
        dimension.insert(*k, true);
        if i == 5 {
          count += 1;
        }
      } else {
        dimension.insert(*k, false);
      }
    }
  }
  count
}

pub fn part_two(input: &str) -> i64 {
  let mut dimension = parse_input2(input);
  let mut count = 0_i64;
  for i in 0..6 {
    // Since we're using a map, we need to get every active cube in the dimension
    // and insert inactive cubes in every adjacent position that doesn't already
    // have a cube.
    let mut new_neighbors: HashSet<Coord4> = HashSet::new();
    for (k, v) in &dimension {
      if !v {
        continue;
      }
      for n in get_neighbors2(&k) {
        new_neighbors.insert(n);
      }
    }
    for k in new_neighbors {
      dimension.entry(k).or_default();
    }

    // Clone the existing dimension so that we can make a replacement.
    let old_dimension = dimension.clone();

    // Build the new dimension from the old one.
    dimension = Dimension4::new();
    for (k, v) in &old_dimension {
      let neighbors = get_neighbors2(&k);
      let mut active = 0;
      for c in neighbors {
        if let Some(n) = old_dimension.get(&c) {
          if *n == true {
            active += 1;
          }
        }
      }
      if active == 3 || (active == 2 && *v == true) {
        dimension.insert(*k, true);
        if i == 5 {
          count += 1;
        }
      } else {
        dimension.insert(*k, false);
      }
    }
  }
  count
}

#[cfg(test)]
mod day17_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        ".#.
..#
###"
      ),
      112
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        ".#.
..#
###"
      ),
      848
    );
  }
}
