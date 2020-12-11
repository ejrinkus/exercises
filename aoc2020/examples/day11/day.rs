#[derive(Clone, Copy, Eq, Debug, PartialEq)]
enum State {
  Floor,
  Empty,
  Occupied,
}

struct SeatMap {
  seats: Vec<State>,
  width: usize,
  height: usize,
}

impl SeatMap {
  pub fn new(input: &str) -> SeatMap {
    if input == "" {
      SeatMap {
        seats: Vec::new(),
        width: 0,
        height: 0,
      }
    } else {
      let mut seat_map: Vec<State> = Vec::new();
      let width = input.find('\n').unwrap();
      let mut height = 0;
      for l in input.lines() {
        for c in l.chars() {
          match c {
            '.' => seat_map.push(State::Floor),
            'L' => seat_map.push(State::Empty),
            '#' => seat_map.push(State::Occupied),
            _ => (),
          }
        }
        height += 1;
      }
      SeatMap {
        seats: seat_map,
        width: width,
        height: height,
      }
    }
  }

  pub fn simulate(&mut self) -> bool {
    let mut changed = false;
    let mut new_seats = self.seats.clone();
    for (i, s) in self.seats.iter().enumerate() {
      if *s == State::Floor {
        continue;
      }
      let adjacents = self.get_adjacents(i);
      let mut neighbors = 0;
      for maybe_a in adjacents.iter() {
        if let Some(j) = maybe_a {
          if self.seats.get(*j) == Some(&State::Occupied) {
            neighbors += 1;
          }
        }
      }
      if neighbors == 0 && *s == State::Empty {
        new_seats[i] = State::Occupied;
        changed = true;
      } else if neighbors >= 4 && *s == State::Occupied {
        new_seats[i] = State::Empty;
        changed = true;
      }
    }
    self.seats = new_seats;
    changed
  }

  pub fn count_occupied(&self) -> u32 {
    self.seats.iter().fold(
      0,
      |acc, s| {
        if *s == State::Occupied {
          acc + 1
        } else {
          acc
        }
      },
    )
  }

  fn get_adjacents(&self, i: usize) -> [Option<usize>; 8] {
    let mut adjacents: [Option<usize>; 8] = [None; 8];
    if i >= self.width {
      if i % self.width != 0 {
        // Up-left
        adjacents[0] = Some(i - self.width - 1);
      }
      // Up
      adjacents[1] = Some(i - self.width);
      if i % self.width != self.width - 1 {
        // Up-right
        adjacents[2] = Some(i - self.width + 1);
      }
    }
    if i < self.seats.len() - self.width {
      if i % self.width != 0 {
        // Down-left
        adjacents[6] = Some(i + self.width - 1);
      }
      // Down
      adjacents[5] = Some(i + self.width);
      if i % self.width != self.width - 1 {
        // Down-right
        adjacents[4] = Some(i + self.width + 1);
      }
    }
    if i % self.width != 0 {
      // Left
      adjacents[7] = Some(i - 1);
    }
    if i % self.width != self.width - 1 {
      // Right
      adjacents[3] = Some(i + 1);
    }
    adjacents
  }
}

pub fn part_one(input: &str) -> u32 {
  let mut seats = SeatMap::new(input);
  while seats.simulate() {}
  seats.count_occupied()
}

pub fn part_two(input: &str) -> i64 {
  println!("{}", input);
  0
}

#[cfg(test)]
mod day11_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
      ),
      37
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
