use prompter;

mod day;

static PART_ONE_INPUT: &'static str = include_str!("part1.txt");
static PART_TWO_INPUT: &'static str = include_str!("part2.txt");

fn main() {
  match prompter::prompter::<u8>("Which part? ") {
    1 => {
      println!("Part one results:");
      println!("{}", day::part_one(PART_ONE_INPUT));
    }
    2 => {
      println!("Part two results:");
      println!("{}", day::part_two(PART_TWO_INPUT));
    }
    _ => println!("Invalid input."),
  }
}
