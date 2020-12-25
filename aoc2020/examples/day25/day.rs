pub fn part_one(input: &str) -> i64 {
  let mut lines = input.lines();
  let card_pub = lines.next().unwrap().parse::<i64>().unwrap();
  let door_pub = lines.next().unwrap().parse::<i64>().unwrap();
  let mut card_loop = 1;
  let mut maybe_pub = 1;
  loop {
    maybe_pub = (maybe_pub * 7) % 20201227;
    if maybe_pub == card_pub {
      break;
    } else {
      card_loop += 1;
    }
  }
  let mut value = 1;
  for _ in 0..card_loop {
    value *= door_pub;
    value %= 20201227;
  }
  value
}

pub fn part_two(input: &str) -> i64 {
  println!("{}", input);
  0
}

#[cfg(test)]
mod day25_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("5764801\n17807724"), 14897079);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two(""), 0);
  }
}
