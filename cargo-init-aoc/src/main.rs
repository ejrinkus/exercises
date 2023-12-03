use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Write};

fn main() {
    // Get the year that we want to initialize.
    let mut input = String::new();
    print!("What year? ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let year = input
        .trim()
        .parse::<u32>()
        .expect("Couldn't parse year into a number");

    // Subdirectory name.
    let dir = format!("aoc{}", year);

    // Maybe update Cargo.toml.
    {
        let file = File::open("Cargo.toml").expect("Couldn't open cargo file");
        let mut reader = BufReader::new(file);

        let mut needs_update = true;
        for line in reader.lines() {
            let parsed = line.expect("Error reading from cargo file");
            if parsed.contains(&dir) {
                println!("{} is already in the workspace", dir);
                needs_update = false;
            }
        }

        if needs_update {
            let f_out =
                File::create("Cargo.toml.tmp").expect("Couldn't create temporary cargo file");
            let f_in = File::open("Cargo.toml").expect("Couldn't open cargo file");
            let mut writer = BufWriter::new(f_out);
            reader = BufReader::new(f_in);
            for line in reader.lines() {
                let parsed = line.expect("Error reading from cargo file");
                write!(writer, "{}\n", parsed).expect("Failed to write line to temp cargo file");
                if parsed.contains("aoc-helpers") {
                    println!("Adding {} to workspace", dir);
                    write!(writer, "\t\"{}\",\n", dir)
                        .expect("Failed to write line to temp cargo file");
                }
            }
            writer.flush().expect("Failed to flush to temp cargo file");
            fs::copy("Cargo.toml.tmp", "Cargo.toml").expect("Failed to copy cargo contents");
            fs::remove_file("Cargo.toml.tmp").expect("Failed to delete temporary cargo file");
        }
    }

    // Create the directory structure.
    println!("creating directory {}/", dir);
    fs::create_dir_all(format!("{}/src/bin", dir)).expect("Failed to create bin directory");
    fs::create_dir_all(format!("{}/src/lib", dir)).expect("Failed to create lib directory");

    // Create the cargo file.
    {
        let mut file =
            File::create(format!("{}/Cargo.toml", dir)).expect("Failed to create aoc cargo file");
        file.write_all(
            format!(
                "[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"Eric Rinkus <ejrinkus@gmail.com>\"]
edition = \"2021\"

[lib]
name = \"aoc_{}_libs\"
path = \"src/lib/lib.rs\"

[[bin]]
name = \"day1\"

[[bin]]
name = \"day2\"

[[bin]]
name = \"day3\"

[[bin]]
name = \"day4\"

[[bin]]
name = \"day5\"

[[bin]]
name = \"day6\"

[[bin]]
name = \"day7\"

[[bin]]
name = \"day8\"

[[bin]]
name = \"day9\"

[[bin]]
name = \"day10\"

[[bin]]
name = \"day11\"

[[bin]]
name = \"day12\"

[[bin]]
name = \"day13\"

[[bin]]
name = \"day14\"

[[bin]]
name = \"day15\"

[[bin]]
name = \"day16\"

[[bin]]
name = \"day17\"

[[bin]]
name = \"day18\"

[[bin]]
name = \"day19\"

[[bin]]
name = \"day20\"

[[bin]]
name = \"day21\"

[[bin]]
name = \"day22\"

[[bin]]
name = \"day23\"

[[bin]]
name = \"day24\"

[[bin]]
name = \"day25\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
aoc-helpers = {{ path = \"../aoc-helpers\" }}
regex = \"1\"
",
                dir, year
            )
            .as_bytes(),
        )
        .expect("Failed to create aoc cargo file");
    }

    // Create the solution files.
    for i in 1..=25 {
        let mut file = File::create(format!("{}/src/bin/day{}.rs", dir, i))
            .expect("Failed to create aoc solution file");
        file.write_all(
            format!(
                "use aoc_helpers::*;

fn main() {{
    let solution = Solution {{}};
    run(&solution);
}}

struct Solution {{}}

impl AocSolution for Solution {{
    fn year(&self) -> u32 {{
        {}
    }}
    fn day(&self) -> u32 {{
        {}
    }}

    fn part_one(&self, _input: &str) -> String {{
        \"\".to_string()
    }}

    fn part_two(&self, _input: &str) -> String {{
        \"\".to_string()
    }}
}}

#[cfg(test)]
mod day1_tests {{
    use super::*;

    #[test]
    fn samples_part1() {{
        let solution = Solution {{}};
        assert_eq!(solution.part_one(\"\"), \"\");
    }}

    #[test]
    fn samples_part2() {{
        let solution = Solution {{}};
        assert_eq!(solution.part_two(\"\"), \"\");
    }}
}}
",
                year, i
            )
            .as_bytes(),
        )
        .expect("Failed to create aoc cargo file");
    }

    // Create an empty lib file.
    File::create(format!("{}/src/lib/lib.rs", dir)).expect("Failed to create aoc solution file");
}
