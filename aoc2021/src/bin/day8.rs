use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        8
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

type Line = ([String; 10], [String; 4]);

fn sort_string(s: &str) -> String {
    let mut bytes: Vec<u8> = s.as_bytes().iter().map(|b| *b).collect();
    bytes.sort();
    String::from_utf8(bytes).unwrap()
}

fn parse_line(line: &str) -> Line {
    let mut parts = line.trim().split(" | ");
    let mut signal_itr = parts.next().unwrap().split(" ");
    let mut output_itr = parts.next().unwrap().split(" ");
    let signals = [
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
        sort_string(signal_itr.next().unwrap()),
    ];
    let outputs = [
        sort_string(output_itr.next().unwrap()),
        sort_string(output_itr.next().unwrap()),
        sort_string(output_itr.next().unwrap()),
        sort_string(output_itr.next().unwrap()),
    ];
    (signals, outputs)
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|l| parse_line(l)).collect()
}

pub fn part_one(input: &str) -> u32 {
    let mut sum = 0u32;
    for (_signals, outputs) in parse_input(input) {
        sum += outputs
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count() as u32;
    }
    sum
}

fn compare_strings(s1: &str, s2: &str) -> u32 {
    let mut shared = 0u32;
    let mut set: HashSet<char> = HashSet::new();
    for c in s1.chars() {
        set.insert(c);
    }
    for c in s2.chars() {
        if set.contains(&c) {
            shared += 1;
        }
    }
    shared
}

fn solve_line(line: &Line) -> u32 {
    let mut ston: HashMap<String, u32> = HashMap::new();
    let mut ntos: HashMap<u32, String> = HashMap::new();
    let signals = &line.0;
    let outputs = &line.1;
    // Get the easy signals.
    for i in 0..10 {
        let signal = &signals[i];
        match signal.len() {
            2 => {
                ston.insert(signal.to_string(), 1);
                ntos.insert(1, signal.to_string());
            }
            3 => {
                ston.insert(signal.to_string(), 7);
                ntos.insert(7, signal.to_string());
            }
            4 => {
                ston.insert(signal.to_string(), 4);
                ntos.insert(4, signal.to_string());
            }
            7 => {
                ston.insert(signal.to_string(), 8);
                ntos.insert(8, signal.to_string());
            }
            _ => continue,
        }
    }
    for i in 0..10 {
        let signal = &signals[i];
        // Below I use `<n1> => <segments>:<n2>` shorthand where <segments> is the
        // number of segments shared by <n1> and <n2>.
        match signal.len() {
            5 => {
                // Could be 2, 3, 5.
                //
                // 2 => 1:1, 2:4, 2:7, 5:8
                // 3 => 2:1, 3:4, 5:7, 5:8
                // 5 => 1:1, 3:4, 2:7, 5:8
                //
                // Therefore:
                //  - if the number shares 2 letters with 4, it's 2.
                //  - if the number shares 2 letters with 1, it's 3.
                //  - otherwise, it's 5.
                if compare_strings(signal, ntos.get(&4).unwrap()) == 2 {
                    ston.insert(signal.to_string(), 2);
                    ntos.insert(2, signal.to_string());
                } else if compare_strings(signal, ntos.get(&1).unwrap()) == 2 {
                    ston.insert(signal.to_string(), 3);
                    ntos.insert(3, signal.to_string());
                } else {
                    ston.insert(signal.to_string(), 5);
                    ntos.insert(5, signal.to_string());
                }
            }
            6 => {
                // Could be 0, 6, 9
                //
                // 0 => 2:1, 3:4, 3:7, 6:8
                // 6 => 1:1, 3:4, 2:7, 6:8
                // 9 => 2:1, 4:4, 3:7, 6:8
                //
                // Therefore:
                //  - if the number shares 1 letter with 1, it's 6.
                //  - if the number shares 4 letters with 4, it's 9.
                //  - otherwise, it's 0.
                if compare_strings(signal, ntos.get(&1).unwrap()) == 1 {
                    ston.insert(signal.to_string(), 6);
                    ntos.insert(6, signal.to_string());
                } else if compare_strings(signal, ntos.get(&4).unwrap()) == 4 {
                    ston.insert(signal.to_string(), 9);
                    ntos.insert(9, signal.to_string());
                } else {
                    ston.insert(signal.to_string(), 0);
                    ntos.insert(0, signal.to_string());
                }
            }
            _ => continue,
        }
    }
    let mut result = 0u32;
    for s in outputs.iter() {
        result *= 10;
        result += ston.get(s).unwrap();
    }
    result
}

pub fn part_two(input: &str) -> u32 {
    parse_input(input).iter().map(|line| solve_line(line)).sum()
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(
      part_one(
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
      ), 26);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(
      part_two(
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
      ), 61229);
    }
}
