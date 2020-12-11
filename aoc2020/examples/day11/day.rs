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
      let adjacents = self.count_adjacents(i);
      if adjacents == 0 && *s == State::Empty {
        new_seats[i] = State::Occupied;
        changed = true;
      } else if adjacents >= 4 && *s == State::Occupied {
        new_seats[i] = State::Empty;
        changed = true;
      }
    }
    self.seats = new_seats;
    changed
  }

  pub fn simulate2(&mut self) -> bool {
    let mut changed = false;
    let mut new_seats = self.seats.clone();
    for (i, s) in self.seats.iter().enumerate() {
      if *s == State::Floor {
        continue;
      }
      let adjacents = self.count_adjacents2(i);
      if adjacents == 0 && *s == State::Empty {
        new_seats[i] = State::Occupied;
        changed = true;
      } else if adjacents >= 5 && *s == State::Occupied {
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

  fn count_adjacents(&self, i: usize) -> u8 {
    let mut adjacents = 0;
    let up_left = i as i32 - self.width as i32 - 1;
    let up = i as i32 - self.width as i32;
    let up_right = i as i32 - self.width as i32 + 1;
    let down_left = i as i32 + self.width as i32 - 1;
    let down = i as i32 + self.width as i32;
    let down_right = i as i32 + self.width as i32 + 1;
    let left = i as i32 - 1;
    let right = i as i32 + 1;

    if up_left >= 0 && i % self.width != 0 {
      adjacents += (*self.seats.get(up_left as usize).unwrap() == State::Occupied) as u8;
    }
    if up >= 0 {
      adjacents += (*self.seats.get(up as usize).unwrap() == State::Occupied) as u8;
    }
    if up_right >= 0 && i % self.width != self.width - 1 {
      adjacents += (*self.seats.get(up_right as usize).unwrap() == State::Occupied) as u8;
    }
    if down_left < self.seats.len() as i32 && i % self.width != 0 {
      adjacents += (*self.seats.get(down_left as usize).unwrap() == State::Occupied) as u8;
    }
    if down < self.seats.len() as i32 {
      adjacents += (*self.seats.get(down as usize).unwrap() == State::Occupied) as u8;
    }
    if down_right < self.seats.len() as i32 && i % self.width != self.width - 1 {
      adjacents += (*self.seats.get(down_right as usize).unwrap() == State::Occupied) as u8;
    }
    if i % self.width != 0 {
      adjacents += (*self.seats.get(left as usize).unwrap() == State::Occupied) as u8;
    }
    if i % self.width != self.width - 1 {
      adjacents += (*self.seats.get(right as usize).unwrap() == State::Occupied) as u8;
    }

    adjacents
  }

  fn count_adjacents2(&self, i: usize) -> u8 {
    let mut adjacents = 0;

    let mut up_left = i as i32 - self.width as i32 - 1;
    let mut up = i as i32 - self.width as i32;
    let mut up_right = i as i32 - self.width as i32 + 1;
    let mut down_left = i as i32 + self.width as i32 - 1;
    let mut down = i as i32 + self.width as i32;
    let mut down_right = i as i32 + self.width as i32 + 1;
    let mut left = i as i32 - 1;
    let mut right = i as i32 + 1;

    while up_left >= 0 && up_left as usize as usize % self.width != self.width - 1 {
      match self.seats.get(up_left as usize).unwrap() {
        State::Floor => up_left = up_left - self.width as i32 - 1,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }
    while up >= 0 {
      match self.seats.get(up as usize).unwrap() {
        State::Floor => up = up - self.width as i32,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }
    while up_right >= 0 && up_right as usize % self.width != 0 {
      match self.seats.get(up_right as usize).unwrap() {
        State::Floor => up_right = up_right - self.width as i32 + 1,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }
    while down_left < self.seats.len() as i32 && down_left as usize % self.width != self.width - 1 {
      match self.seats.get(down_left as usize).unwrap() {
        State::Floor => down_left = down_left + self.width as i32 - 1,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }
    while down < self.seats.len() as i32 {
      match self.seats.get(down as usize).unwrap() {
        State::Floor => down = down + self.width as i32,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }
    while down_right < self.seats.len() as i32 && down_right as usize % self.width != 0 {
      match self.seats.get(down_right as usize).unwrap() {
        State::Floor => down_right = down_right + self.width as i32 + 1,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }
    while left >= 0 && left as usize % self.width != self.width - 1 {
      match self.seats.get(left as usize).unwrap() {
        State::Floor => left = left - 1,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }
    while (right as usize) < self.seats.len() && right as usize % self.width != 0 {
      match self.seats.get(right as usize).unwrap() {
        State::Floor => right = right + 1,
        State::Empty => break,
        State::Occupied => {
          adjacents += 1;
          break;
        }
      }
    }

    adjacents
  }
}

pub fn part_one(input: &str) -> u32 {
  let mut seats = SeatMap::new(input);
  while seats.simulate() {}
  seats.count_occupied()
}

pub fn part_two(input: &str) -> u32 {
  let mut seats = SeatMap::new(input);
  while seats.simulate2() {}
  seats.count_occupied()
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
    assert_eq!(
      part_two(
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
      26
    );
  }
}
