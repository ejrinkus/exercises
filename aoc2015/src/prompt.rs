use std::fs;
use std::io::{self, Write};
use std::vec::Vec;

pub fn prompter(year: &str, day: &str) -> (u8, String) {
  // Enter the data directory for this day, to make it easier to access input
  // files.
  let dir = format!("./aoc{}/examples/day{}/data", year, day);
  let err_msg = format!("Failed to enter {}", dir);
  std::env::set_current_dir(dir).expect(&err_msg);
  
  // Figure out whether we run part 1 or 2.
  let mut input = String::new();
  print!("Which part? ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).expect("failed to read part number");
  let part_num: u8 = input.trim()
    .parse::<u8>()
    .unwrap();
  
  // Provide the user a list of the available input files, and ask them which
  // one to use.
  input = String::new();
  let paths: Vec<String> = fs::read_dir("./")
    .unwrap()
    .map(|x| x.unwrap()
    .path()
    .file_name()
    .unwrap()
    .to_os_string()
    .into_string()
    .unwrap())
    .collect();
  println!("Which file?");
  for (i, p) in paths.iter().enumerate() {
    println!("  {}. {}", i, p);
  }
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).expect("failed to read filename");
  let file_num: usize = input.trim()
    .parse::<usize>()
    .unwrap();
  
  (part_num, paths[file_num].clone())
}