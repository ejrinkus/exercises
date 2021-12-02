use aoc2021;
use chrono::{Datelike, FixedOffset};
use futures::executor::block_on;

extern crate reqwest;
use reqwest::header;

use std::io::{self, Write};

static AOC_TZ_OFFSET: &'static i32 = &(5 * 3600);

#[tokio::main]
async fn main() {
    // Iniitalize year and day using today as the default.
    let now = chrono::offset::Local::now().with_timezone(&FixedOffset::west(*AOC_TZ_OFFSET));
    let year = prompt::<i32>("Which year", now.date().year());
    let day = prompt::<u32>("Which day", now.date().day());
    let part = prompt::<u32>("Which part", 1);
    println!("Selected year {}, day {}, part {}", year, day, part);

    // Get input.
    let input = block_on(get_input(year, day));
    run_year(year, day, part, &input);
}

fn prompt<T>(msg: &str, default: T) -> T
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    print!("{} (default {})? ", msg, default);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    if input.trim().is_empty() {
        return default;
    }
    input.trim().parse::<T>().unwrap()
}

async fn get_input(year: i32, day: u32) -> String {
    let mut headers = header::HeaderMap::new();
    headers.insert("cookie", "session=53616c7465645f5fe9df7ddefd3d66a94d0ee4bffb4ffb1a413d7cc8660ba3f9678af6676b437609597602b7f2f21b46".parse().unwrap());

    reqwest::Client::new()
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

fn run_year(year: i32, day: u32, part: u32, input: &str) {
    if year == 2021 {
        aoc2021::run(day, part, input);
    }
}
