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
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> u32 {
    let max_red = 12u32;
    let max_green = 13u32;
    let max_blue = 14u32;
    let mut sum = 0u32;
    input.lines().for_each(|l| {
        let results = parse(l);
        if results.1 <= max_red && results.2 <= max_green && results.3 <= max_blue {
            sum += results.0;
        }
    });
    sum
}

pub fn part_two(input: &str) -> u32 {
    let mut sum = 0u32;
    input.lines().for_each(|l| {
        let results = parse(l);
        sum += results.1 * results.2 * results.3;
    });
    sum
}

fn parse(game: &str) -> (u32, u32, u32, u32) {
    let id_and_draws = game.split(':').collect::<Vec<&str>>();
    let id_str = id_and_draws[0].split(' ').collect::<Vec<&str>>()[1];

    let draws = id_and_draws[1].split(';').collect::<Vec<&str>>();

    let red_re: Regex = Regex::new(r"(\d+) red").unwrap();
    let green_re: Regex = Regex::new(r"(\d+) green").unwrap();
    let blue_re: Regex = Regex::new(r"(\d+) blue").unwrap();

    let mut max_red = 0u32;
    let mut max_green = 0u32;
    let mut max_blue = 0u32;
    for draw in draws {
        let maybe_red = red_re.captures(draw);
        if maybe_red.is_some() {
            let red = maybe_red.expect("red should've been Some")[1]
                .parse::<u32>()
                .expect("didn't get number for red");
            max_red = if red > max_red { red } else { max_red };
        }
        let maybe_green = green_re.captures(draw);
        if maybe_green.is_some() {
            let green = maybe_green.expect("green should've been Some")[1]
                .parse::<u32>()
                .expect("didn't get number for green");
            max_green = if green > max_green { green } else { max_green };
        }
        let maybe_blue = blue_re.captures(draw);
        if maybe_blue.is_some() {
            let blue = maybe_blue.expect("blue should've been Some")[1]
                .parse::<u32>()
                .expect("didn't get number for blue");
            max_blue = if blue > max_blue { blue } else { max_blue };
        }
    }

    (
        id_str
            .parse::<u32>()
            .expect("uh-oh, didn't get a number for the id"),
        max_red,
        max_green,
        max_blue,
    )
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
            part_one(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
            part_two(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
