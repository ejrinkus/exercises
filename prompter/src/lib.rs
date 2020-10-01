use std::io::{self, Write};

pub fn prompter<T>(msg: &str) -> T 
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug, {
  // Figure out whether we run part 1 or 2.
  let mut input = String::new();
  print!("{}", msg);
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).expect("failed to read input");
  let output: T = input.trim()
    .parse::<T>()
    .unwrap();

  output
}