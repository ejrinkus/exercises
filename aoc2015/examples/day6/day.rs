use std::vec::Vec;

static _WIDTH: usize = 1000;
static _HEIGHT: usize = 1000;

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
  ON,
  OFF,
  TOGGLE,
}

pub fn part_one(input: &str) -> i64 {
  let mut lights = vec![0; _WIDTH * _HEIGHT];
  for line in input.lines() {
    let (p1, p2, inst_type) = parse_instruction(line);
    for x in p1.0 as usize..=p2.0 as usize {
      for y in p1.1 as usize..=p2.1 as usize {
        let i = p_to_i(&(x, y));
        match &inst_type {
          Instruction::ON => lights[i] = 1,
          Instruction::OFF => lights[i] = 0,
          Instruction::TOGGLE => lights[i] = if lights[i] == 0 { 1 } else { 0 },
        }
      }
    }
  }
  lights.iter().sum()
}

pub fn part_two(input: &str) -> i64 {
  let mut lights = vec![0; _WIDTH * _HEIGHT];
  for line in input.lines() {
    let (p1, p2, inst_type) = parse_instruction(line);
    for x in p1.0 as usize..=p2.0 as usize {
      for y in p1.1 as usize..=p2.1 as usize {
        let i = p_to_i(&(x, y));
        match &inst_type {
          Instruction::ON => lights[i] += 1,
          Instruction::OFF => lights[i] = if lights[i] > 0 { lights[i] - 1 } else { 0 },
          Instruction::TOGGLE => lights[i] += 2,
        }
      }
    }
  }
  lights.iter().sum()
}

pub fn parse_instruction(inst: &str) -> ((i64, i64), (i64, i64), Instruction) {
  let inst_type: Instruction;
  let prefix: &str;
  if inst.starts_with("turn on ") {
    prefix = "turn on ";
    inst_type = Instruction::ON;
  } else if inst.starts_with("turn off ") {
    prefix = "turn off ";
    inst_type = Instruction::OFF;
  } else if inst.starts_with("toggle ") {
    prefix = "toggle ";
    inst_type = Instruction::TOGGLE;
  } else {
    panic!("WTF, NO MATCHING PREFIX");
  }
  let coords: Vec<i64> = inst
    .trim_start_matches(prefix)
    .split(" through ")
    .map(|x| x.split(','))
    .flatten()
    .map(|x| x.parse::<i64>().unwrap())
    .collect();
  if coords.len() != 4 {
    panic!("WTF, TOO MANY NUMBERS");
  }

  ((coords[0], coords[1]), (coords[2], coords[3]), inst_type)
}

pub fn p_to_i(p: &(usize, usize)) -> usize {
  p.0 + (p.1 * _WIDTH)
}

#[cfg(test)]
mod day6_tests {
  use super::*;

  #[test]
  fn parse_test() {
    let cases = [
      (
        "turn off 674,321 through 793,388",
        (674, 321),
        (793, 388),
        Instruction::OFF,
      ),
      (
        "toggle 749,672 through 973,965",
        (749, 672),
        (973, 965),
        Instruction::TOGGLE,
      ),
      (
        "turn on 943,30 through 990,907",
        (943, 30),
        (990, 907),
        Instruction::ON,
      ),
    ];

    for case in cases.iter() {
      let result = parse_instruction(case.0);
      assert_eq!(result.0, case.1);
      assert_eq!(result.1, case.2);
      assert_eq!(result.2, case.3);
    }
  }

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("turn on 0,0 through 999,999"), 1000000);
    assert_eq!(
      part_one("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"),
      999996
    );
    assert_eq!(part_one("toggle 0,0 through 999,0"), 1000);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("turn on 0,0 through 999,999"), 1000000);
    assert_eq!(
      part_two("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"),
      999996
    );
    assert_eq!(part_two("toggle 0,0 through 999,0"), 2000);
  }
}
