use itertools::join;
use num::traits::PrimInt;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Tile {
  id: u64,
  contents: String,
  // Order: left, right, top, bottom,
  //        left_flipped, right_flipped, top_flipped, bottom_flipped.
  edges: [u128; 8],
  width: usize,
  height: usize,
}

impl FromStr for Tile {
  type Err = std::string::ParseError;

  fn from_str(tile_str: &str) -> Result<Self, Self::Err> {
    let width = tile_str.find('\n').unwrap();
    let height = tile_str.lines().count();
    if width != 10 {
      panic!("unexpected line length: {}", width);
    }
    let top: u128 = tile_str
      .lines()
      .next()
      .unwrap()
      .char_indices()
      .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
      .sum();
    let bottom: u128 = tile_str
      .lines()
      .last()
      .unwrap()
      .char_indices()
      .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
      .sum();
    let left: u128 = tile_str
      .lines()
      .map(|l| l.chars().next().unwrap())
      .enumerate()
      .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
      .sum();
    let right: u128 = tile_str
      .lines()
      .map(|l| l.chars().last().unwrap())
      .enumerate()
      .map(|(i, c)| if c == '#' { 1 << (width - i - 1) } else { 0 })
      .sum();
    Ok(Tile {
      id: 0,
      contents: tile_str.to_owned(),
      edges: [
        left,
        right,
        top,
        bottom,
        binary_flip(left, height),
        binary_flip(right, height),
        binary_flip(top, width),
        binary_flip(bottom, width),
      ],
      width,
      height,
    })
  }
}

impl Tile {
  // Flip the tile vertically in place (along the horizontal axis).
  pub fn flip_v(&mut self) {
    self.contents = join(self.contents.lines().rev(), "\n");
    self.edges = [self.edges[4],
                  self.edges[5],
                  self.edges[2],
                  self.edges[3],
                  self.edges[0],
                  self.edges[1],
                  self.edges[6],
                  self.edges[7]];
  }
  // Flip the tile horizontally in place (along the vertical axis).
  pub fn flip_x(&mut self) {
    self.contents = join(self.contents.lines().rev(), "\n");
    self.edges = [self.edges[0],
                  self.edges[1],
                  self.edges[6],
                  self.edges[7],
                  self.edges[4],
                  self.edges[5],
                  self.edges[2],
                  self.edges[3]];
  }
  // Rotate the tile 90 degrees clockwise in place.
  pub fn rot_right(&mut self) {
    let mut new_contents = String::new();
    for i in 0..self.height {
      let new_row: String = self.contents.lines().fold(String::new(), |mut acc, s| {
        acc.push(s.chars().skip(i).next().unwrap());
        acc
      }).chars().rev().collect();
      new_contents.push('\n');
      new_contents.push_str(&new_row);
    }
    self.contents = new_contents;
    let temp = self.height;
    self.height = self.width;
    self.width = temp;
    self.edges = [self.edges[3],  // Bottom moves to left
                  self.edges[2],  // Top moves to right
                  self.edges[4],  // Left (flipped) moves to top
                  self.edges[5],  // Right (flipped) moves to bottom 
                  self.edges[7],  // Bottom (flipped) moves to left (flipped)
                  self.edges[6],  // Top (flipped) moves to right (flipped)
                  self.edges[0],  // Left moves to top (flipped)
                  self.edges[1]]  // Right moves to bottom (flipped)
  }
  // Rotate the tile 90 degrees counterclockwise in place.
  pub fn rot_left(&mut self) {
    let mut new_contents = String::new();
    for i in 0..self.height {
      let new_row: String = self.contents.lines().fold(String::new(), |mut acc, s| {
        acc.push(s.chars().skip(self.height - i - 1).next().unwrap());
        acc
      }).chars().collect();
      new_contents.push('\n');
      new_contents.push_str(&new_row);
    }
    self.contents = new_contents;
  }
  // Rotate the tile 180 degrees in place.
  pub fn rot_180(&mut self) {}
  // Attempt to stitch |tile| to this tile.  |border| is the matching border value
  // between the two tiles.  Takes ownership of |tile|.
  pub fn stitch(&mut self, tile: Tile, border: u128) {
    let side = self.edges.iter().enumerate().find(|(_, v)| **v == border).unwrap().0;
  }
}

// A tile's border can be easily converted to a 10 bit number.  A BoT maps these
// border-values with the IDs that have them.
type BtoT = HashMap<u128, Vec<Tile>>;

// Takes an unsigned integer |num| with |len| bits and reverses the bits.  Note that
// |len| does not correspond to capacity of |num| in bits.  E.g. |num| may be a u16,
// but if |len| is 10 then we only treat it as a 10-bit number.
pub fn binary_flip<'a, B>(mut num: B, mut len: usize) -> B
where
  B: PrimInt,
{
  let mut ret: B = B::zero();
  // We're dealing with 10-bit numbers, even though they're stored in u16.
  // For part 2 I need to start stitching together tiles, so
  // they may have borders longer than 10.
  while num > B::zero() {
    ret = ret.unsigned_shl(1);
    ret = ret | (num & B::one());
    num = num.unsigned_shr(1);
    len = len - 1;
  }
  ret = ret.unsigned_shl(len as u32);
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
    let mut tile = tile_body.parse::<Tile>().unwrap();
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
