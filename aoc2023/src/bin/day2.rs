use aoc_helpers::*;
use regex::Regex;

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
        2
    }

    fn part_one(&self, input: &str) -> String {
        let mut sum = 0u32;
        input.lines().for_each(|l| {
            let results = max_colors_for_game(l);
            if results.red <= 12 && results.green <= 13 && results.blue <= 14 {
                sum += results.id;
            }
        });
        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut sum = 0u32;
        input.lines().for_each(|l| {
            let results = max_colors_for_game(l);
            sum += results.red * results.blue * results.green;
        });
        sum.to_string()
    }
}

struct ColorsResponse {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn max_colors_for_game(game: &str) -> ColorsResponse {
    let id_re: Regex = Regex::new(r"^Game (\d+)").expect("Failed to compile id regex");
    let id = id_re
        .captures(game)
        .expect("Failed to get captures on id regex")
        .get(1)
        .expect("Failed to retrieve first capture group from id regex")
        .as_str()
        .parse::<u32>()
        .expect("Failed to parse id to number");

    let red_re: Regex = Regex::new(r"(\d+) red").expect("Failed to compile red regex");
    let max_red = red_re
        .captures_iter(game)
        .map(|c| c.extract::<1usize>().1[0].parse::<u32>().unwrap())
        .max()
        .expect("Failed to find max red count");

    let green_re: Regex = Regex::new(r"(\d+) green").expect("Failed to compile green regex");
    let max_green = green_re
        .captures_iter(game)
        .map(|c| c.extract::<1usize>().1[0].parse::<u32>().unwrap())
        .max()
        .expect("Failed to find max green count");

    let blue_re: Regex = Regex::new(r"(\d+) blue").expect("Failed to compile blue regex");
    let max_blue = blue_re
        .captures_iter(game)
        .map(|c| c.extract::<1usize>().1[0].parse::<u32>().unwrap())
        .max()
        .expect("Failed to find max blue count");

    ColorsResponse {
        id: id,
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "8"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "2286"
        );
    }
}
