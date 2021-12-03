use isahc::{prelude::*, send, Request};
use std::io::{self, Write};

pub fn get_input(year: i32, day: u32) -> String {
    let mut response = send(
        Request::get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("cookie", "session=53616c7465645f5fe9df7ddefd3d66a94d0ee4bffb4ffb1a413d7cc8660ba3f9678af6676b437609597602b7f2f21b46")
        .body(())
        .unwrap())
    .unwrap();

    response.text().unwrap()
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
