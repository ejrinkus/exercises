use aoc_helpers::*;
use std::vec::Vec;

const YEAR: u32 = 2021;
const DAY: u32 = 11;

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

struct OctoGrid {
  grid: Vec<Vec<u8>>,
  width: usize,
  height: usize,
  bright_bois: Vec<(usize, usize)>,
  flashes: u64
}

impl OctoGrid {
  pub fn new(input: &str) -> OctoGrid {
    let mut octos = Vec::new();
    for line in input.lines() {
      let mut row: Vec<u8> = Vec::new();
      for c in line.chars() {
        row.push(c.to_digit(10).unwrap() as u8);
      }
      octos.push(row);
    }
    let (w, h) = (octos[0].len(), octos.len());
    OctoGrid {
      grid: octos,
      width: w,
      height: h,
      bright_bois: Vec::new(),
      flashes: 0
    }
  }

  fn increment(&mut self) {
    for (y, row) in self.grid.iter_mut().enumerate() {
      for (x, octo) in row.iter_mut().enumerate() {
        *octo += 1;
        if *octo > 9 {
          self.bright_bois.push((x, y));
        }
      }
    }
  }

  fn flash(&mut self) {
    let mut i = 0usize;
    while i < self.bright_bois.len() {
      let (x, y) = self.bright_bois[i];
      if x > 0 {
        if y > 0 {
          self.grid[y-1][x-1] += 1;
          if self.grid[y-1][x-1] == 10 {
            self.bright_bois.push((x-1, y-1));
          }
        }
        self.grid[y][x-1] += 1;
        if self.grid[y][x-1] == 10 {
          self.bright_bois.push((x-1, y));
        }
        if y < self.height-1 {
          self.grid[y+1][x-1] += 1;
          if self.grid[y+1][x-1] == 10 {
            self.bright_bois.push((x-1, y+1));
          }
        }
      }
      if x < self.width-1 {
        if y > 0 {
          self.grid[y-1][x+1] += 1;
          if self.grid[y-1][x+1] == 10 {
            self.bright_bois.push((x+1, y-1));
          }
        }
        self.grid[y][x+1] += 1;
        if self.grid[y][x+1] == 10 {
          self.bright_bois.push((x+1, y));
        }
        if y < self.height-1 {
          self.grid[y+1][x+1] += 1;
          if self.grid[y+1][x+1] == 10 {
            self.bright_bois.push((x+1, y+1));
          }
        }
      }
      if y > 0 {
        self.grid[y-1][x] += 1;
        if self.grid[y-1][x] == 10 {
          self.bright_bois.push((x, y-1));
        }
      }
      if y < self.height-1 {
        self.grid[y+1][x] += 1;
        if self.grid[y+1][x] == 10 {
          self.bright_bois.push((x, y+1));
        }
      }
      i += 1;
    }
    while !self.bright_bois.is_empty() {
      let (x, y) = self.bright_bois.pop().unwrap();
      self.grid[y][x] = 0;
      self.flashes += 1;
    }
  }

  pub fn step(&mut self) {
    self.increment();
    self.flash();
  }
}

pub fn part_one(input: &str) -> u64 {
  let mut octos = OctoGrid::new(input);
  for _i in 0..100 {
    octos.step();
  }
  octos.flashes
}

pub fn part_two(input: &str) -> i64 {
  let mut octos = OctoGrid::new(input);
  let total_octos = (octos.width as u64) * (octos.height as u64);
  let mut step = 1;
  let mut flashes_before = 0;
  loop {
    octos.step();
    if octos.flashes - flashes_before == total_octos {
      break;
    }
    flashes_before = octos.flashes;
    step += 1;
  }
  step
}

#[cfg(test)]
mod day2_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"), 1656);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"), 195);
  }
}
