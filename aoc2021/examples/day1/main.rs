use prompter;

extern crate reqwest;
use reqwest::header;

mod day;

static PART_ONE_INPUT: &'static str = include_str!("part1.txt");
static PART_TWO_INPUT: &'static str = include_str!("part2.txt");

#[tokio::main]
async fn main() {
  let mut headers = header::HeaderMap::new();
  headers.insert("cookie", "session=53616c7465645f5fe9df7ddefd3d66a94d0ee4bffb4ffb1a413d7cc8660ba3f9678af6676b437609597602b7f2f21b46".parse().unwrap());

  let res = reqwest::Client::new()
    .get("https://adventofcode.com/2021/day/1/input")
    .headers(headers)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  match prompter::prompter::<u8>("Which part? ") {
    1 => {
      println!("Part one results:");
      println!("{}", day::part_one(&res));
    }
    2 => {
      println!("Part two results:");
      println!("{}", day::part_two(&res));
    }
    _ => println!("Invalid input."),
  }
}
