use aoc_helpers::runner::*;
use nom::character::complete::*;
use nom::IResult;

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
        7
    }

    fn part_one(&self, _input: &str) -> String {
        "".to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        "".to_string()
    }
}

fn parse_hand(s: &str) -> ([usize; 13], u32) {
    let mut hand = [0usize; 13];
    let mut bid = 0;

    (hand, bid)
}

fn take_space(s: &str) -> IResult<&str, &str> {
    space1(s)
}

fn take_card(s: &str) -> IResult<&str, char> {
    one_of("AKQJT98765432")(s)
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            ""
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(solution.part_two(""), "");
    }
}
