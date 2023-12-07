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
    let mut rem: &str;
    let mut taken: &str;

    (rem, _) = take_tag(game, "Game ").expect("Game didn't start with \"Game\"");
    (rem, taken) = take_number(rem).expect("Missing game ID");
    let id = taken.parse::<u32>().expect("Malformed game ID");

    let mut max_colors = vec![0, 0, 0]; // RGB
    while rem != "" {
        (rem, _) = take_n(rem, 2).expect("Missing delimiters");
        (rem, taken) = take_number(rem).expect("Missing cube count");
        let count = taken.parse::<u32>().expect("Malformed count");
        (rem, _) = take_n(rem, 1).expect("Missing space after count");
        if let Ok((maybe_rem, _)) = take_tag(rem, "red") {
            if max_colors[0] < count {
                max_colors[0] = count;
            }
            rem = maybe_rem;
        } else if let Ok((maybe_rem, _)) = take_tag(rem, "green") {
            if max_colors[1] < count {
                max_colors[1] = count;
            }
            rem = maybe_rem;
        } else if let Ok((maybe_rem, _)) = take_tag(rem, "blue") {
            if max_colors[2] < count {
                max_colors[2] = count;
            }
            rem = maybe_rem;
        } else {
            panic!("Didn't get a valid color");
        }
    }

    ColorsResponse {
        id: id,
        red: max_colors[0],
        green: max_colors[1],
        blue: max_colors[2],
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
