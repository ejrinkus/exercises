use isahc::{prelude::*, send, Request};
use std::io::{self, Write};

const SESSION_COOKIE: &str = include_str!("session.cookie");

pub trait AocSolution {
    fn year(&self) -> u32;
    fn day(&self) -> u32;
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn run<T: AocSolution>(solution: &T) {
    println!(
        "Running AoC {} day {} solution...",
        solution.year(),
        solution.day()
    );
    let input = get_input(solution.year(), solution.day());
    if prompt_for_part(1) {
        let result = solution.part_one(&input);
        println!("{}", result);
        if prompt_to_submit() {
            println!(
                "{}",
                submit_answer(solution.year(), solution.day(), 1, &result)
            );
        }
    }
    if prompt_for_part(2) {
        let result = solution.part_two(&input);
        println!("{}", result);
        if prompt_to_submit() {
            println!(
                "{}",
                submit_answer(solution.year(), solution.day(), 2, &result)
            );
        }
    }
}

fn get_input(year: u32, day: u32) -> String {
    let mut response = send(
        Request::get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("cookie", format!("session={}", SESSION_COOKIE))
        .body(())
        .unwrap(),
    )
    .unwrap();

    response.text().unwrap()
}

fn submit_answer(year: u32, day: u32, part: u32, answer: &str) -> String {
    let mut response = send(
        Request::post(format!(
            "https://adventofcode.com/{}/day/{}/answer",
            year, day
        ))
        .header("cookie", format!("session={}", SESSION_COOKIE))
        .body(format!("level={}&answer={}", part, answer))
        .unwrap(),
    )
    .unwrap();

    response
        .text()
        .unwrap()
        .split("<main>")
        .skip(1)
        .next()
        .unwrap()
        .split("</main>")
        .next()
        .unwrap()
        .to_string()
}

fn prompt_for_part(part: u8) -> bool {
    let mut input = String::new();
    print!("Run part {} (y/N)? ", part);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let output = input.trim();

    output == "y" || output == "Y" || output == "yes"
}

fn prompt_to_submit() -> bool {
    let mut input = String::new();
    print!("Submit answer (y/N)? ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let output = input.trim();

    output == "y" || output == "Y" || output == "yes"
}

pub fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(input.lines().count());
    for l in input.lines() {
        let row: Vec<char> = l.chars().collect();
        matrix.push(row);
    }
    matrix
}
