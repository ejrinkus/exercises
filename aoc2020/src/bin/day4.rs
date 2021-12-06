use aoc_helpers::*;
use regex::Regex;

const YEAR: u32 = 2020;
const DAY: u32 = 4;

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

const FIELD_PREFIXES: [(&'static str, u8, fn(&str) -> bool); 8] = [
  ("byr", 7, validate_byr),
  ("iyr", 6, validate_iyr),
  ("eyr", 5, validate_eyr),
  ("hgt", 4, validate_hgt),
  ("hcl", 3, validate_hcl),
  ("ecl", 2, validate_ecl),
  ("pid", 1, validate_pid),
  ("cid", 0, validate_cid),
];

fn validate_byr(val: &str) -> bool {
  if val.len() != 4 {
    return false;
  }
  let maybe_year = val.parse::<u16>();
  if maybe_year.is_err() {
    return false;
  }
  let year = maybe_year.unwrap();
  year >= 1920 && year <= 2002
}

fn validate_iyr(val: &str) -> bool {
  if val.len() != 4 {
    return false;
  }
  let maybe_year = val.parse::<u16>();
  if maybe_year.is_err() {
    return false;
  }
  let year = maybe_year.unwrap();
  year >= 2010 && year <= 2020
}

fn validate_eyr(val: &str) -> bool {
  if val.len() != 4 {
    return false;
  }
  let maybe_year = val.parse::<u16>();
  if maybe_year.is_err() {
    return false;
  }
  let year = maybe_year.unwrap();
  year >= 2020 && year <= 2030
}

fn validate_hgt(val: &str) -> bool {
  if val.ends_with("cm") {
    let maybe_height = val.trim_end_matches("cm").parse::<u16>();
    if maybe_height.is_err() {
      return false;
    }
    let height = maybe_height.unwrap();
    height >= 150 && height <= 193
  } else if val.ends_with("in") {
    let maybe_height = val.trim_end_matches("in").parse::<u16>();
    if maybe_height.is_err() {
      return false;
    }
    let height = maybe_height.unwrap();
    height >= 59 && height <= 76
  } else {
    false
  }
}

fn validate_hcl(val: &str) -> bool {
  let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  re.is_match(val)
}

fn validate_ecl(val: &str) -> bool {
  match val {
    "amb" => true,
    "blu" => true,
    "brn" => true,
    "gry" => true,
    "grn" => true,
    "hzl" => true,
    "oth" => true,
    _ => false,
  }
}

fn validate_pid(val: &str) -> bool {
  let re = Regex::new(r"^[0-9]{9}$").unwrap();
  re.is_match(val)
}

fn validate_cid(_val: &str) -> bool {
  true
}

pub fn check_passports(input: &str, also_validate: bool) -> i64 {
  let mut count_valid = 0;
  // Bit mask representing fields we've found.  Bits are in this order:
  // byr, iyr, eyr, hgt, hcl, ecl, pid, cid.
  let mut fields: u8 = 0;
  for line in input.lines() {
    // If we hit an empty line, we just finished a passport and gotta start a
    // new one.
    if line.is_empty() {
      if fields == std::u8::MAX || fields == std::u8::MAX - 1 {
        count_valid += 1;
      }
      fields = 0;
      continue;
    }

    // We're starting a new passport.
    for piece in line.split(' ') {
      let prefix = &piece[0..3];
      let val = &piece[4..];
      for pair in FIELD_PREFIXES.iter() {
        if prefix == pair.0 {
          if !also_validate || pair.2(val) {
            fields |= 1 << pair.1;
          }
        }
      }
    }
  }

  // We still need to check the final passport.
  if fields == std::u8::MAX || fields == std::u8::MAX - 1 {
    count_valid += 1;
  }
  count_valid
}

pub fn part_one(input: &str) -> i64 {
  check_passports(input, false)
}

pub fn part_two(input: &str) -> i64 {
  check_passports(input, true)
}

#[cfg(test)]
mod day4_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
      ),
      2
    );
  }

  #[test]
  fn samples_part2_invalid() {
    assert_eq!(
      part_two(
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
      ),
      0
    );
  }

  #[test]
  fn samples_part2_valid() {
    assert_eq!(
      part_two(
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
      ),
      1
    );

    assert_eq!(
      part_two(
        "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
      ),
      1
    );

    assert_eq!(
      part_two(
        "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022"
      ),
      1
    );

    assert_eq!(
      part_two("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
      1
    );
  }
}
