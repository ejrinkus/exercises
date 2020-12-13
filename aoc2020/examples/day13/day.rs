pub fn part_one(input: &str) -> u64 {
  let mut lines = input.lines();
  let depart = lines.next().unwrap().parse::<u64>().unwrap();
  let ids = lines.next().unwrap().split(',');
  let mut min_id = 0_u64;
  let mut min_diff = u64::MAX;
  for id in ids {
    if id == "x" {
      continue;
    }
    let id_num = id.parse::<u64>().unwrap();
    if depart % id_num == 0 {
      return 0; // There's a bus that leaves at the perfect time.
    }
    let id_diff = (((depart / id_num) * id_num) + id_num) - depart;
    if id_diff < min_diff {
      min_id = id_num;
      min_diff = id_diff;
    }
  }
  min_id * min_diff
}

pub fn find_ts(offset: u64, id: u64, factor: u64, prev: &Vec<(u64, u64)>) -> u64 {
  let mut maybe_ts = factor;
  loop {
    let mut solved = true;
    for (o, n) in prev {
      if (maybe_ts + o) % n != 0 {
        solved = false;
        break;
      }
    }
    if solved && (maybe_ts + offset) % id == 0 {
      return maybe_ts;
    }
    maybe_ts += factor;
  }
}

pub fn part_two(input: &str) -> u64 {
  let mut lines = input.lines();
  lines.next();
  let ids = lines.next().unwrap().split(',');
  let mut enumerated_ids: Vec<(u64, u64)> = Vec::new();
  for (offset, id) in ids.enumerate() {
    if id == "x" {
      continue;
    }
    enumerated_ids.push((offset as u64, id.parse::<u64>().unwrap()));
  }
  enumerated_ids.sort_by(|a, b| a.1.cmp(&b.1));
  let mut solved_ids: Vec<(u64, u64)> = Vec::new();
  let mut timestamp = 1;
  for (o, id) in enumerated_ids {
    timestamp = find_ts(o, id, timestamp, &solved_ids);
    solved_ids.push((o, id));
    println!("up to id {}, offset {}, solution is {}", id, o, timestamp);
  }
  timestamp
  // // LCM of primes is the product of primes.  The target timestamp must be less
  // // than the LCM of all the bus IDs
  // let lcm = enumerated_ids.iter().fold(1, |acc, (_o, n)| acc * n);
  // // Similarly, the target timestamp must be _greater_ than the LCM of the bus
  // // IDs _except_ for the largest.
  // let lcm_lower = enumerated_ids
  //   .iter()
  //   .take(enumerated_ids.len() - 1)
  //   .fold(1, |acc, (_o, n)| acc * n);
  // println!("range ({}, {})", lcm - lcm_lower, lcm);

  // let max_o = enumerated_ids.last().unwrap().0;
  // let max = enumerated_ids.last().unwrap().1;
  // let min = enumerated_ids.first().unwrap().1;
  // let mut timestamp = ((lcm - lcm_lower) / max) * max;
  // while timestamp < lcm {
  //   let mut solved = true;
  //   for (o, n) in &enumerated_ids {
  //     if (timestamp - max_o + *o as u64) % n != 0 {
  //       solved = false;
  //       break;
  //     }
  //   }
  //   if solved {
  //     return timestamp - max_o;
  //   }
  //   timestamp += max;
  // }
  // 0
}

#[cfg(test)]
mod day13_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "939
7,13,x,x,59,x,31,19"
      ),
      295
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        "939
7,13,x,x,59,x,31,19"
      ),
      1068781
    );
  }
}

/*
target: 1068781

lcm: 3162341
lcm_lower: 166439
*/
