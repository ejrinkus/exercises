use aoc_helpers::*;
use std::collections::HashSet;
use std::vec::Vec;

const YEAR: u32 = 2021;
const DAY: u32 = 13;

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

enum Direction {
  X,
  Y
}

struct Page {
  dots: HashSet<(usize, usize)>,
  folds: Vec<(Direction, usize)>,
}

impl Page {
  pub fn new(input: &str) -> Page {
    let mut lines = input.lines();
    let mut dotline = lines.next().unwrap().trim();
    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    while dotline != "" {
      let mut coords = dotline.split(",");
      let (x, y) = (coords.next().unwrap(), coords.next().unwrap());
      dots.insert((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
      dotline = lines.next().unwrap().trim();
    }
    let mut folds: Vec<(Direction, usize)> = Vec::new();
    for line in lines {
      let replaced = line.replace("fold along ", "");
      let mut parts = replaced.split("=");
      let dir = parts.next().unwrap();
      let val = parts.next().unwrap().parse::<usize>().unwrap();
      match dir {
        "x" => folds.push((Direction::X, val)),
        "y" => folds.push((Direction::Y, val)),
        _ => panic!("unexpected direction value {}", dir)
      }
    }
    Page {
      dots: dots,
      folds: folds
    }
  }

  pub fn fold_once(&mut self, i: usize) {
    let (dir, val) = &self.folds[i];
    let mut newdots: HashSet<(usize, usize)> = HashSet::new();
    for (x, y) in &self.dots {
      match dir {
        Direction::X => {
          if x < val {
            newdots.insert((*x, *y));
          } else {
            newdots.insert(((2*val)-x, *y));
          }
        },
        Direction::Y => {
          if y < val {
            newdots.insert((*x, *y));
          } else {
            newdots.insert((*x, (2*val)-y));
          }
        }
      }
    }
    self.dots = newdots;
  }

  pub fn fold_all(&mut self) {
    for i in 0..self.folds.len() {
      self.fold_once(i);
    }
  }
}

pub fn part_one(input: &str) -> usize {
  let mut page = Page::new(input);
  page.fold_once(0);
  page.dots.len()
}

pub fn part_two(input: &str) -> usize {
  let mut page = Page::new(input);
  page.fold_all();
  let mut grid: Vec<Vec<char>> = Vec::new();
  for (x, y) in &page.dots {
    while grid.len() <= *y {
      grid.push(Vec::new());
    }
    while grid[*y].len() <= *x {
      grid[*y].push('.');
    }
    grid[*y][*x] = '#';
  }
  for row in grid {
    for c in row {
      print!("{}", c);
    }
    println!("");
  }
  page.dots.len()
}

#[cfg(test)]
mod day13_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"), 17);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"), 16);
  }
}
