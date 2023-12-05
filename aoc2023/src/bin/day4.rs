use aoc_helpers::runner::*;
use std::collections::HashSet;

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
        4
    }

    fn part_one(&self, input: &str) -> String {
        let mut sum = 0u32;
        for line in input.lines() {
            let matches = parse_card(line);
            if matches > 0 {
                sum += 2u32.pow(matches - 1);
            }
        }
        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut card_copies: Vec<u32> = Vec::with_capacity(203usize);
        card_copies.push(0);
        for (i, line) in input.lines().enumerate() {
            if i >= card_copies.len() {
                card_copies.push(0);
            }
            card_copies[i] += 1;
            let matches = parse_card(line);
            if matches == 0 {
                continue;
            }
            for j in i + 1..i + (matches as usize) + 1 {
                if j >= card_copies.len() {
                    card_copies.push(0);
                }
                card_copies[j] += card_copies[i];
            }
        }
        card_copies.iter().sum::<u32>().to_string()
    }
}

fn parse_card(card: &str) -> u32 {
    let pieces: Vec<&str> = card.split(' ').collect();
    let mut points = 0u32;
    let mut switch = false;
    let mut winning_nums: HashSet<u32> = HashSet::new();
    for piece in pieces {
        if piece == "|" {
            switch = true;
            continue;
        }
        if let Ok(num) = piece.parse::<u32>() {
            if !switch {
                winning_nums.insert(num);
            } else if winning_nums.contains(&num) {
                points += 1;
            }
        }
    }
    points
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "13"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "30"
        );
    }
}
