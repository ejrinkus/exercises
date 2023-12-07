use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        6
    }

    fn part_one(&self, input: &str) -> String {
        let mut iter = input.lines();
        let time_line = iter.next().unwrap();
        let dist_line = iter.next().unwrap();

        let mut rem: &str;
        let mut taken: &str;

        (rem, _) = take_tag(time_line, "Time:").expect("Missing Times label");

        let mut times: Vec<u64> = Vec::new();
        while rem != "" {
            (rem, _) = take_spaces(rem).expect("Missing spaces before time");
            (rem, taken) = take_number(rem).expect("Missing time");
            times.push(taken.parse::<u64>().expect("Malformed time"));
        }

        (rem, _) = take_tag(dist_line, "Distance:").expect("Missing Times label");

        let mut distances: Vec<u64> = Vec::new();
        while rem != "" {
            (rem, _) = take_spaces(rem).expect("Missing spaces before distance");
            (rem, taken) = take_number(rem).expect("Missing distance");
            distances.push(taken.parse::<u64>().expect("Malformed distance"));
        }

        let mut product = 1u64;
        for i in 0..times.len() {
            product *= calc_combos(times[i] as f64, distances[i] as f64) as u64;
        }

        product.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut iter = input.lines();
        let time_line = iter.next().unwrap();
        let dist_line = iter.next().unwrap();

        let mut rem: &str;
        let mut taken: &str;

        (rem, _) = take_tag(time_line, "Time:").expect("Missing Times label");

        let mut time_segments: Vec<&str> = Vec::new();
        while rem != "" {
            (rem, _) = take_spaces(rem).expect("Missing spaces before time");
            (rem, taken) = take_number(rem).expect("Missing time");
            time_segments.push(taken);
        }
        let time = time_segments
            .join("")
            .parse::<u64>()
            .expect("Malformed time");

        (rem, _) = take_tag(dist_line, "Distance:").expect("Missing Times label");

        let mut distance_segments: Vec<&str> = Vec::new();
        while rem != "" {
            (rem, _) = take_spaces(rem).expect("Missing spaces before distance");
            (rem, taken) = take_number(rem).expect("Missing distance");
            distance_segments.push(taken);
        }
        let distance = distance_segments
            .join("")
            .parse::<u64>()
            .expect("Malformed distance");

        let result = calc_combos(time as f64, distance as f64) as u64;

        result.to_string()
    }
}

fn calc_combos(time: f64, distance: f64) -> f64 {
    let neg_time = 0f64 - time;
    let time_sqrd = time * time;
    let pre_root = time_sqrd - (4f64 * distance);
    let exact_hold = (neg_time + pre_root.sqrt()) / -2f64;
    let mut min_hold = exact_hold.ceil();
    if exact_hold == min_hold {
        min_hold += 1f64;
    }
    let max_hold = time - min_hold;
    max_hold - min_hold + 1f64
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            "288"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            "71503"
        );
    }
}
