use itertools::join;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Tile {
  id: u64,
  contents: String,
  // Order: left, right, top, bottom,
  //        left_flipped, right_flipped, top_flipped, bottom_flipped.
  edges: [u16; 8],
}

// A tile's border can be easily converted to a 10 bit number.  A BoT maps these
// border-values with the IDs that have them.
type BtoT = HashMap<u16, Vec<Tile>>;

// Order of output doesn't matter since the tiles might be rotated.  The left
// and right borders are computed top-down.
pub fn parse_tile(tile_str: &str) -> Tile {
  let width = tile_str.find('\n').unwrap();
  if width != 10 {
    panic!("unexpected line length: {}", width);
  }
  let top: u16 = tile_str
    .lines()
    .next()
    .unwrap()
    .char_indices()
    .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
    .sum();
  let bottom: u16 = tile_str
    .lines()
    .last()
    .unwrap()
    .char_indices()
    .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
    .sum();
  let left: u16 = tile_str
    .lines()
    .map(|l| l.chars().next().unwrap())
    .enumerate()
    .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
    .sum();
  let right: u16 = tile_str
    .lines()
    .map(|l| l.chars().last().unwrap())
    .enumerate()
    .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
    .sum();
  Tile {
    id: 0,
    contents: tile_str.to_owned(),
    edges: [
      left,
      right,
      top,
      bottom,
      binary_flip(left),
      binary_flip(right),
      binary_flip(top),
      binary_flip(bottom),
    ],
  }
}

pub fn binary_flip(mut num: u16) -> u16 {
  let mut ret = 0;
  // We're dealing with 10-bit numbers, even though they're stored in u16.
  for _ in 0..10 {
    ret <<= 1;
    ret |= num & 1;
    num >>= 1;
  }
  ret
}

// for every tile, convert the 4 edges into 10-bit values, _and_ reverse those
// values to represent the flipped version of tile.  Then map each of those 8
// border values back to the tile's ID.  And pray that the input guarantees
// no two of these 8 values will ever be the same.
pub fn parse_input(input: &str) -> BtoT {
  let mut ret = BtoT::new();
  for tile_str in input.split("\n\n") {
    let mut lines = tile_str.lines();
    let id = lines
      .next()
      .unwrap()
      .strip_prefix("Tile ")
      .unwrap()
      .strip_suffix(":")
      .unwrap()
      .parse::<u64>()
      .unwrap();
    let tile_body = join(lines, "\n");
    let mut tile = parse_tile(&tile_body);
    tile.id = id;
    for border in tile.edges.iter() {
      ret.entry(*border).or_insert(Vec::new()).push(tile.clone());
    }
  }
  ret
}

pub fn part_one(input: &str) -> u64 {
  let btot = parse_input(input);
  // Assuming we can rely on the fact that any given border value will only
  // be used by two tiles (that are adjacent on that border), we just need to
  // find the 4 tiles that only pair with 2 other tiles.
  let mut per_tile_matches: HashMap<u64, usize> = HashMap::new();
  for (_border, tiles) in btot {
    if tiles.len() == 2 {
      // This is a matching border, update each ID's count.
      for tile in tiles {
        *per_tile_matches.entry(tile.id).or_insert(0) += 1;
      }
    }
  }
  let mut product = 1;
  // Since we included flipped borders in our btot, each tile pairing actually
  // appears twice: once for regular, once for flipped.  So we actually expect
  // corner tiles to have a count of 4, edge tiles to have a count of 6, and
  // middle tiles a count of 8.
  for (id, count) in per_tile_matches {
    if count == 4 {
      product *= id;
    }
  }
  product
}

pub fn part_two(input: &str) -> i64 {
  println!("{}", input);
  0
}

#[cfg(test)]
mod day20_tests {
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
