mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run(day: u32, part: u32, input: &str) {
    match day {
        1 => {
            if part == 1 {
                println!("Part one: {}", day1::part_one(input));
            } else {
                println!("Part two: {}", day1::part_two(input));
            }
        }
        2 => {
            if part == 1 {
                println!("Part one: {}", day2::part_one(input));
            } else {
                println!("Part two: {}", day2::part_two(input));
            }
        }
        3 => {
            if part == 1 {
                println!("Part one: {}", day3::part_one(input));
            } else {
                println!("Part two: {}", day3::part_two(input));
            }
        }
        4 => {
            if part == 1 {
                println!("Part one: {}", day4::part_one(input));
            } else {
                println!("Part two: {}", day4::part_two(input));
            }
        }
        5 => {
            if part == 1 {
                println!("Part one: {}", day5::part_one(input));
            } else {
                println!("Part two: {}", day5::part_two(input));
            }
        }
        6 => {
            if part == 1 {
                println!("Part one: {}", day6::part_one(input));
            } else {
                println!("Part two: {}", day6::part_two(input));
            }
        }
        7 => {
            if part == 1 {
                println!("Part one: {}", day7::part_one(input));
            } else {
                println!("Part two: {}", day7::part_two(input));
            }
        }
        8 => {
            if part == 1 {
                println!("Part one: {}", day8::part_one(input));
            } else {
                println!("Part two: {}", day8::part_two(input));
            }
        }
        9 => {
            if part == 1 {
                println!("Part one: {}", day9::part_one(input));
            } else {
                println!("Part two: {}", day9::part_two(input));
            }
        }
        10 => {
            if part == 1 {
                println!("Part one: {}", day10::part_one(input));
            } else {
                println!("Part two: {}", day10::part_two(input));
            }
        }
        11 => {
            if part == 1 {
                println!("Part one: {}", day11::part_one(input));
            } else {
                println!("Part two: {}", day11::part_two(input));
            }
        }
        12 => {
            if part == 1 {
                println!("Part one: {}", day12::part_one(input));
            } else {
                println!("Part two: {}", day12::part_two(input));
            }
        }
        13 => {
            if part == 1 {
                println!("Part one: {}", day13::part_one(input));
            } else {
                println!("Part two: {}", day13::part_two(input));
            }
        }
        14 => {
            if part == 1 {
                println!("Part one: {}", day14::part_one(input));
            } else {
                println!("Part two: {}", day14::part_two(input));
            }
        }
        15 => {
            if part == 1 {
                println!("Part one: {}", day15::part_one(input));
            } else {
                println!("Part two: {}", day15::part_two(input));
            }
        }
        16 => {
            if part == 1 {
                println!("Part one: {}", day16::part_one(input));
            } else {
                println!("Part two: {}", day16::part_two(input));
            }
        }
        17 => {
            if part == 1 {
                println!("Part one: {}", day17::part_one(input));
            } else {
                println!("Part two: {}", day17::part_two(input));
            }
        }
        18 => {
            if part == 1 {
                println!("Part one: {}", day18::part_one(input));
            } else {
                println!("Part two: {}", day18::part_two(input));
            }
        }
        19 => {
            if part == 1 {
                println!("Part one: {}", day19::part_one(input));
            } else {
                println!("Part two: {}", day19::part_two(input));
            }
        }
        20 => {
            if part == 1 {
                println!("Part one: {}", day20::part_one(input));
            } else {
                println!("Part two: {}", day20::part_two(input));
            }
        }
        21 => {
            if part == 1 {
                println!("Part one: {}", day21::part_one(input));
            } else {
                println!("Part two: {}", day21::part_two(input));
            }
        }
        22 => {
            if part == 1 {
                println!("Part one: {}", day22::part_one(input));
            } else {
                println!("Part two: {}", day22::part_two(input));
            }
        }
        23 => {
            if part == 1 {
                println!("Part one: {}", day23::part_one(input));
            } else {
                println!("Part two: {}", day23::part_two(input));
            }
        }
        24 => {
            if part == 1 {
                println!("Part one: {}", day24::part_one(input));
            } else {
                println!("Part two: {}", day24::part_two(input));
            }
        }
        25 => {
            if part == 1 {
                println!("Part one: {}", day25::part_one(input));
            } else {
                println!("Part two: {}", day25::part_two(input));
            }
        }
        _ => {
            return;
        }
    }
}
