use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        9
    }

    fn part_one(&self, input: &str) -> String {
        let mut sum = 0i64;
        for line in input.lines() {
            let original_sequence = read_sequence(line);
            let mut new_val = original_sequence[original_sequence.len() - 1];
            let (mut new_seq, mut is_terminal) = gen_next_sequence(&original_sequence);
            new_val += new_seq[new_seq.len() - 1];
            while !is_terminal {
                (new_seq, is_terminal) = gen_next_sequence(&new_seq);
                new_val += new_seq[new_seq.len() - 1];
            }
            sum += new_val;
        }
        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut sum = 0i64;
        for line in input.lines() {
            let original_sequence = read_sequence(line);
            let mut new_val = original_sequence[0];
            let (mut new_seq, mut is_terminal) = gen_next_sequence(&original_sequence);
            new_val -= new_seq[0];
            let mut is_sub = false;
            while !is_terminal {
                (new_seq, is_terminal) = gen_next_sequence(&new_seq);
                if is_sub {
                    new_val -= new_seq[0];
                } else {
                    new_val += new_seq[0];
                }
                is_sub = !is_sub;
            }
            sum += new_val;
        }
        sum.to_string()
    }
}

fn read_sequence(s: &str) -> Vec<i64> {
    let mut rem = s;
    let mut val: i64;
    let mut vals: Vec<i64> = Vec::new();
    while rem != "" {
        (rem, val) = take_i64(rem).expect("Didn't get a number");
        vals.push(val);
        let result = take_spaces(rem);
        if result.is_ok() {
            rem = result.unwrap().0;
        } else {
            break;
        }
    }
    vals
}
fn gen_next_sequence(seq: &Vec<i64>) -> (Vec<i64>, bool) {
    if seq.len() == 1 {
        return (vec![0], true);
    }
    let mut is_terminal = true;
    let mut prev = seq[0];
    let mut new_seq: Vec<i64> = Vec::with_capacity(seq.len() - 1);
    for v in seq.iter().skip(1) {
        let diff = v - prev;
        is_terminal &= diff == 0;
        new_seq.push(diff);
        prev = *v;
    }
    (new_seq, is_terminal)
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "114"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "2"
        );
    }
}
