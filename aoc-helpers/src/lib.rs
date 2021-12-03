use isahc::{prelude::*, send, Request};
use std::io::{self, Write};

const SESSION_COOKIE: &str = "53616c7465645f5fe9df7ddefd3d66a94d0ee4bffb4ffb1a413d7cc8660ba3f9678af6676b437609597602b7f2f21b46";

pub fn get_input(year: u32, day: u32) -> String {
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

pub fn submit_answer(year: u32, day: u32, part: u32, answer: &str) -> String {
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

pub fn prompt_for_part(part: u8) -> bool {
    let mut input = String::new();
    print!("Run part {} (y/N)? ", part);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let output = input.trim();

    output == "y" || output == "Y" || output == "yes"
}

pub fn prompt_to_submit() -> bool {
    let mut input = String::new();
    print!("Submit answer (y/N)? ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let output = input.trim();

    output == "y" || output == "Y" || output == "yes"
}
