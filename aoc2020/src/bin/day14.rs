use aoc_helpers::*;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2020
    }
    fn day(&self) -> u32 {
        14
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn update_bitmask(mask: &str) -> (u64, u64) {
    let mut mask_zeroes = u64::MAX;
    let mut mask_ones = 0_u64;
    for (i, c) in mask.chars().rev().enumerate() {
        if c == 'X' {
            continue;
        } else if c == '0' {
            mask_zeroes ^= 1 << i;
        } else if c == '1' {
            mask_ones |= 1 << i;
        } else {
            panic!("unexpected value in mask: {}", mask);
        }
    }
    (mask_zeroes, mask_ones)
}

pub fn part_one(input: &str) -> u64 {
    let mask_regex = Regex::new(r"^mask = ([X01]+)").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let (mut mask_ones, mut mask_zeroes) = (0_u64, 0_u64);
    for line in input.lines() {
        if let Some(caps) = mask_regex.captures(line) {
            let masks = update_bitmask(&caps[1]);
            mask_zeroes = masks.0;
            mask_ones = masks.1;
        } else if let Some(caps) = mem_regex.captures(line) {
            let address = (&caps[1]).parse::<u64>().unwrap();
            let val = (&caps[2]).parse::<u64>().unwrap();
            let new_val = memory.entry(address).or_insert(0);
            *new_val = (val & mask_zeroes) | mask_ones;
        }
    }
    memory.values().sum()
}

pub fn mask_address(mask: &str, addr: String) -> String {
    mask.chars()
        .zip(addr.chars())
        .map(|(m, a)| {
            if m == 'X' {
                'X'
            } else if m == '1' {
                '1'
            } else {
                a
            }
        })
        .collect()
}

pub fn part_two(input: &str) -> u64 {
    let mask_regex = Regex::new(r"^mask = ([X01]+)").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "000000000000000000000000000000000000".to_owned();
    for line in input.lines() {
        if let Some(caps) = mask_regex.captures(line) {
            mask = caps[1].to_owned();
        } else if let Some(caps) = mem_regex.captures(line) {
            // Get the values out of the regex match.
            let floated_address = mask_address(
                &mask,
                format!("{:036b}", (&caps[1]).parse::<u64>().unwrap()),
            );
            let val = (&caps[2]).parse::<u64>().unwrap();

            // Count the number of X's in the floated address.
            let mut addresses: Vec<u64> = Vec::new();
            let floats = floated_address.match_indices('X').count();

            // Loop over the values 0 -> 2^float.  The binary representation of these
            // values can be subbed in for the X's in the floated address.
            for float_mask in 0..2_u64.pow(floats as u32) {
                let mut expanded = floated_address.clone();
                let bitstr_short = format!("{:b}", float_mask);
                let mut bitstr = String::new();
                for _i in 0..(floats - bitstr_short.len()) {
                    bitstr.push_str("0");
                }
                bitstr.push_str(&bitstr_short);
                for bit in bitstr.chars() {
                    expanded = expanded.replacen('X', &bit.to_string(), 1);
                }
                addresses.push(u64::from_str_radix(&expanded, 2).unwrap());
            }
            if floats == 0 {
                // If there are no X's in the mask, then the 'floated_address' is
                // actually just a regular address that we can use directly.
                addresses.push(u64::from_str_radix(&floated_address, 2).unwrap());
            }

            // After expanding our addresses, update each one with the new value.
            for addr in addresses {
                let new_val = memory.entry(addr).or_insert(0);
                *new_val = val;
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            ),
            165
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            ),
            208
        );
    }
}
