use aoc2015::prompt;
use std::fs;

fn main() {
    let (part, file) = prompt::prompter("2015", "1");
    let contents = fs::read_to_string(file).expect("failed to read file");
    if part == 1 {
      println!("Part one results:");
      part_one(&contents);
    } else if part == 2 {
      println!("Part two results:");
      part_two(&contents);
    }
}

fn part_one(input: &str) {
  let pos = input.chars().filter(|x| *x == '(').count() as i64;
  let neg = input.chars().filter(|x| *x == ')').count() as i64;
  println!("{}", pos - neg);
}

fn part_two(input: &str) {
  let mut floor: i64 = 0;
  let mut count: usize = 1;
  for c in input.chars() {
    if c == '(' {
      floor += 1;
    } else if c == ')' {
      floor -= 1;
    }
    if floor < 0 {
      break;
    }
    count += 1;
  }
  println!("{}", count);
}