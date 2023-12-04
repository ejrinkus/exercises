use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::vec::Vec;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        10
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

fn get_corrupted_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_incomplete_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn is_match(open: char, close: char) -> bool {
    (open == '(' && close == ')')
        || (open == '{' && close == '}')
        || (open == '[' && close == ']')
        || (open == '<' && close == '>')
}

fn find_match(open: char) -> char {
    match open {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => '\0',
    }
}

fn is_open(c: char) -> bool {
    c == '(' || c == '{' || c == '[' || c == '<'
}

fn is_corrupted(line: &str) -> (Vec<char>, bool) {
    let mut open_stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if is_open(c) {
            open_stack.push(c);
        } else {
            match open_stack.pop() {
                Some(open) => {
                    if !is_match(open, c) {
                        return (vec![c], true);
                    }
                }
                None => return (vec![c], true),
            }
        }
    }
    (open_stack, false)
}

pub fn part_one(input: &str) -> u64 {
    let mut total = 0u64;
    for line in input.lines() {
        let (stack, is_corrupted) = is_corrupted(line);
        if is_corrupted {
            total += get_corrupted_score(stack[0]);
        }
    }
    total
}

pub fn part_two(input: &str) -> u64 {
    let mut totals: Vec<u64> = Vec::new();
    for line in input.lines() {
        let (mut stack, is_corrupted) = is_corrupted(line);
        if is_corrupted {
            continue;
        }
        let mut total = 0u64;
        while !stack.is_empty() {
            let open = stack.pop().unwrap();
            let close = find_match(open);
            total = (total * 5) + get_incomplete_score(close);
        }
        totals.push(total);
    }
    totals.sort();
    totals[totals.len() / 2]
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            ),
            26397
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            ),
            288957
        );
    }
}
