use itertools::join;
use num::traits::PrimInt;
use regex::Regex;
use std::collections::{HashMap, HashSet};
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
                  self.edges[3],
                  self.edges[2],
                  self.edges[0],
                  self.edges[1],
                  self.edges[7],
                  self.edges[6]];
  }
  // Flip the tile horizontally in place (along the vertical axis).
  pub fn flip_x(&mut self) {
    self.contents = join(self.contents.lines().map(|l| l.chars().rev().collect::<String>()), "\n");
    self.edges = [self.edges[1],
                  self.edges[0],
                  self.edges[6],
                  self.edges[7],
                  self.edges[5],
                  self.edges[4],
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
      new_contents.push_str(&new_row);
      if i != self.height - 1 {
        new_contents.push('\n');
      }
    }
    self.contents = new_contents;
    let temp = self.height;
    self.height = self.width;
    self.width = temp;
    self.edges = [self.edges[3],
                  self.edges[2],
                  self.edges[4],
                  self.edges[5],
                  self.edges[7],
                  self.edges[6],
                  self.edges[0],
                  self.edges[1]];
  }
  // Rotate the tile 90 degrees counterclockwise in place.
  pub fn rot_left(&mut self) {
    let mut new_contents = String::new();
    for i in 0..self.height {
      let new_row: String = self.contents.lines().fold(String::new(), |mut acc, s| {
        acc.push(s.chars().skip(self.height - i - 1).next().unwrap());
        acc
      }).chars().collect();
      new_contents.push_str(&new_row);
      if i != self.height - 1{
        new_contents.push('\n');
      }
    }
    self.contents = new_contents;
    let temp = self.height;
    self.height = self.width;
    self.width = temp;
    self.edges = [self.edges[6],
                  self.edges[7],
                  self.edges[1],
                  self.edges[0],
                  self.edges[2],
                  self.edges[3],
                  self.edges[5],
                  self.edges[4]];
  }
  // Rotate the tile 180 degrees in place.
  pub fn rot_180(&mut self) {
    self.contents = self.contents.chars().rev().collect();
    self.edges = [self.edges[5],
                  self.edges[4],
                  self.edges[7],
                  self.edges[6],
                  self.edges[1],
                  self.edges[0],
                  self.edges[3],
                  self.edges[2]];
  }
  // Attempt to stitch |source| to this tile.  |border| is the matching border value
  // between the two tiles.  Takes ownership of |source|.
  pub fn stitch(&mut self, mut source: Tile, border: u128) {
    // Put the matching side on the right for this tile.
    let self_side = self.edges.iter().enumerate().find(|(_, v)| **v == border).unwrap().0;
    match self_side {
      0 => { self.flip_x(); },
      1 => { },
      2 => { self.rot_right(); },
      3 => { self.rot_left(); self.flip_v(); },
      4 => { self.rot_180(); },
      5 => { self.flip_v(); },
      6 => { self.rot_left(); self.flip_x(); },
      7 => { self.rot_left(); },
      _ => { panic!("invalid side index {}", border); },
    }
    // Put the matching side on the left for the source tile.
    let source_side = source.edges.iter().enumerate().find(|(_, v)| **v == border).unwrap().0;
    match source_side {
      0 => { },
      1 => { source.flip_x(); },
      2 => { source.rot_left(); source.flip_v(); },
      3 => { source.rot_right(); },
      4 => { source.flip_v(); },
      5 => { source.rot_180(); },
      6 => { source.rot_left(); },
      7 => { source.rot_left(); source.flip_x(); },
      _ => { panic!("invalid side index {}", border); },
    }
    // Update dimensions & edges.
    self.width += source.width;
    self.edges[1] = source.edges[1];
    self.edges[2] = (self.edges[2] << source.width) | source.edges[2];
    self.edges[3] = (self.edges[3] << source.width) | source.edges[3];
    self.edges[5] = source.edges[5];
    self.edges[6] = (source.edges[6] << source.width) | self.edges[6];
    self.edges[7] = (source.edges[7] << source.width) | self.edges[7];
    // Stitch the content.
    self.contents = join(self.contents.lines().zip(source.contents.lines()).map(|(s1, s2)| format!("{}{}", s1, s2)), "\n");
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

pub fn part_two(input: &str) -> u64 {
  let mut btot = parse_input(input);
  while btot.len() > 8 {
    // Iterate over all border matches in btot.  Stitch each matching pair, then put it into
    // next_btot to repeat in the next iteration.  Once btot only contains 8 entries, that
    // means we have the final tile (the tile is in the map 8 times, once for each border combo).
    let mut next_btot = BtoT::new();
    let mut found: HashSet<u64> = HashSet::new();
    for (border, tiles) in btot {
      if tiles.len() < 2 {
        // Skip outer edges.
        continue;
      }
      let mut tile1 = tiles[0].clone();
      let mut tile2 = tiles[1].clone();
      if found.contains(&tile1.id) || found.contains(&tile2.id) {
        // Skip tiles that have already been stitched once this iteration.
        if !found.contains(&tile1.id) {
          for e in &tile1.edges {
            next_btot.entry(*e).or_insert(Vec::new()).push(tile1.clone());
          }
          found.insert(tile1.id);
        }
        if !found.contains(&tile2.id) {
          for e in &tile2.edges {
            next_btot.entry(*e).or_insert(Vec::new()).push(tile2.clone());
          }
          found.insert(tile2.id);
        }
        continue;
      }
      found.insert(tile1.id);
      found.insert(tile2.id);
      tile1.stitch(tile2, border);
      for e in &tile1.edges {
        next_btot.entry(*e).or_insert(Vec::new()).push(tile1.clone());
      }
    }
    btot = next_btot;
  }
  let mut final_tile = btot.values_mut().next().unwrap().pop().unwrap();
  println!("{}", final_tile.contents);
  let mut count = 0;
  let mut i = 0;
  let sm1 = Regex::new(r"..................#.").unwrap();
  let sm2 = Regex::new(r"#....##....##....###").unwrap();
  let sm3 = Regex::new(r".#..#..#..#..#..#...").unwrap();
  while i < final_tile.contents.lines().count() - 2 {
    let line1 = final_tile.contents.lines().nth(i).unwrap();
    let line2 = final_tile.contents.lines().nth(i+1).unwrap();
    let line3 = final_tile.contents.lines().nth(i+2).unwrap();
    if let Some(m1) = sm1.find(line1) {
      if let Some(m2) = sm2.find(line2) {
        if let Some(m3) = sm3.find(line3) {
          if m1.start() == m2.start() && m2.start() == m3.start() {
            count += 1;
            i += 2;  // Prevent overlaps.
          }
        }
      }
    }
    i += 1;
  }
  count
}

#[cfg(test)]
mod day20_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."), 20899048083289);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }

  #[test]
  fn tile_test() {
    let mut tile = Tile::from_str(".##..#...#
##.#......
.##......#
#.....#..#
........##
.#.......#
###.#.....
###.....##
#..#....#.
..####..##").unwrap();
    assert_eq!(tile.edges[0], 334);
    assert_eq!(tile.edges[1], 757);
    assert_eq!(tile.edges[2], 401);
    assert_eq!(tile.edges[3], 243);
    assert_eq!(tile.edges[4], 458);
    assert_eq!(tile.edges[5], 701);
    assert_eq!(tile.edges[6], 550);
    assert_eq!(tile.edges[7], 828);

    tile.flip_v();
    assert_eq!(tile.contents, "..####..##
#..#....#.
###.....##
###.#.....
.#.......#
........##
#.....#..#
.##......#
##.#......
.##..#...#");
    assert_eq!(tile.edges[0], 458);
    assert_eq!(tile.edges[1], 701);
    assert_eq!(tile.edges[2], 243);
    assert_eq!(tile.edges[3], 401);
    assert_eq!(tile.edges[4], 334);
    assert_eq!(tile.edges[5], 757);
    assert_eq!(tile.edges[6], 828);
    assert_eq!(tile.edges[7], 550);

    tile.flip_x();
    assert_eq!(tile.contents, "##..####..
.#....#..#
##.....###
.....#.###
#.......#.
##........
#..#.....#
#......##.
......#.##
#...#..##.");
    assert_eq!(tile.edges[0], 701);
    assert_eq!(tile.edges[1], 458);
    assert_eq!(tile.edges[2], 828);
    assert_eq!(tile.edges[3], 550);
    assert_eq!(tile.edges[4], 757);
    assert_eq!(tile.edges[5], 334);
    assert_eq!(tile.edges[6], 243);
    assert_eq!(tile.edges[7], 401);

    tile.rot_right();
    assert_eq!(tile.contents, "#.####.#.#
....#..###
..........
...#......
#........#
......#..#
.#......##
#.#...##.#
###..###..
.#.#..###.");
    assert_eq!(tile.edges[0], 550);
    assert_eq!(tile.edges[1], 828);
    assert_eq!(tile.edges[2], 757);
    assert_eq!(tile.edges[3], 334);
    assert_eq!(tile.edges[4], 401);
    assert_eq!(tile.edges[5], 243);
    assert_eq!(tile.edges[6], 701);
    assert_eq!(tile.edges[7], 458);

    tile.rot_left();
    assert_eq!(tile.contents, "##..####..
.#....#..#
##.....###
.....#.###
#.......#.
##........
#..#.....#
#......##.
......#.##
#...#..##.");
    assert_eq!(tile.edges[0], 701);
    assert_eq!(tile.edges[1], 458);
    assert_eq!(tile.edges[2], 828);
    assert_eq!(tile.edges[3], 550);
    assert_eq!(tile.edges[4], 757);
    assert_eq!(tile.edges[5], 334);
    assert_eq!(tile.edges[6], 243);
    assert_eq!(tile.edges[7], 401);

    tile.rot_180();
    assert_eq!(tile.contents, ".##..#...#
##.#......
.##......#
#.....#..#
........##
.#.......#
###.#.....
###.....##
#..#....#.
..####..##");
    assert_eq!(tile.edges[0], 334);
    assert_eq!(tile.edges[1], 757);
    assert_eq!(tile.edges[2], 401);
    assert_eq!(tile.edges[3], 243);
    assert_eq!(tile.edges[4], 458);
    assert_eq!(tile.edges[5], 701);
    assert_eq!(tile.edges[6], 550);
    assert_eq!(tile.edges[7], 828);

    let tile2 = Tile::from_str("#.####.#.#
##.#......
.##......#
#.....#..#
........#.
.#.......#
###.#.....
###.....##
#..#....#.
..####..##").unwrap();
    tile.stitch(tile2, 757);
    assert_eq!(tile.contents, ".##..#...###.#..###.
##.#.......##..###..
.##......##.#...##.#
#.....#..###......##
........###.....#..#
.#.......##........#
###.#........#......
###.....###.........
#..#....#.....#..###
..####..###.##.#.#.#");
    assert_eq!(tile.width, 20);
    assert_eq!(tile.height, 10);
    assert_eq!(tile.edges[0], 334);
    assert_eq!(tile.edges[1], 243);
    assert_eq!(tile.edges[2], 411470);
    assert_eq!(tile.edges[3], 249557);
    assert_eq!(tile.edges[4], 458);
    assert_eq!(tile.edges[5], 828);
    assert_eq!(tile.edges[6], 470566);
    assert_eq!(tile.edges[7], 702268);
  }
}
