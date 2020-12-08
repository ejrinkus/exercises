pub fn sum_json_nums(input: &str) -> i64 {
  let mut sum = 0_i64;
  let mut number = String::new();
  for c in input.chars() {
    if c == '-' || c.is_numeric() {
      number.push(c);
    } else if !number.is_empty() {
      sum += number.parse::<i64>().unwrap();
      number = String::new();
    }
  }
  sum
}

pub fn remove_red(input: &str) -> String {
  let mut output: String = String::from(input);
  loop {
    if !output.contains(":\"red\"") {
      break;
    }
    let mut chunks = output.splitn(2, ":\"red\"");

    // Step 1: find the last '{' in the first chunk; this is the start of the
    // object containing this "red" field.
    let chunk1 = chunks.next().unwrap();

    let mut braces = 0;
    let mut count1 = chunk1.len();
    for c in chunk1.chars().rev() {
      if c == '{' && braces == 0 {
        break;
      }
      match c {
        '}' => braces += 1,
        '{' => braces -= 1,
        _ => (),
      }
      count1 -= 1;
    }

    // Step 2: find the '}' that balances the '{' found in the previous step;
    // this is the end of the object containing this "red" field.
    let chunk2 = chunks.next().unwrap();

    braces = 0;
    let mut count2 = 0;
    for c in chunk2.chars() {
      if c == '}' && braces == 0 {
        break;
      }
      match c {
        '{' => braces += 1,
        '}' => braces -= 1,
        _ => (),
      }
      count2 += 1;
    }

    // Step 3: trim the object out of the output.
    output = chunk1
      .chars()
      .take(count1)
      .chain(chunk2.chars().skip(count2))
      .collect();
  }
  output
}

pub fn part_one(input: &str) -> i64 {
  sum_json_nums(input)
}

pub fn part_two(input: &str) -> i64 {
  sum_json_nums(&remove_red(input))
}

#[cfg(test)]
mod day12_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(part_one("[1,2,3]"), 6);
    assert_eq!(part_one("{\"a\":2,\"b\":4}"), 6);
    assert_eq!(part_one("[[[3]]]"), 3);
    assert_eq!(part_one("{\"a\":{\"b\":4},\"c\":-1}"), 3);
    assert_eq!(part_one("{\"a\":[-1,1]}"), 0);
    assert_eq!(part_one("[-1,{\"a\":1}]"), 0);
    assert_eq!(part_one("[]"), 0);
    assert_eq!(part_one("{}"), 0);
  }

  #[test]
  fn samples_part2() {
    assert_eq!(part_two("[1,2,3]"), 6);
    assert_eq!(part_two("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
    assert_eq!(part_two("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
    assert_eq!(part_two("[1,\"red\",5]"), 6);
  }
}
