use aoc_helpers::*;

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
        17
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn parse_input(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut parts = line.trim_start_matches("target area: ").split(", ");
    let mut xparts = parts.next().unwrap().trim_start_matches("x=").split("..");
    let mut yparts = parts.next().unwrap().trim_start_matches("y=").split("..");
    let (xstart, xend) = (
        xparts.next().unwrap().parse::<i32>().unwrap(),
        xparts.next().unwrap().parse::<i32>().unwrap(),
    );
    let (ystart, yend) = (
        yparts.next().unwrap().parse::<i32>().unwrap(),
        yparts.next().unwrap().parse::<i32>().unwrap(),
    );
    ((xstart, xend), (ystart, yend))
}

pub fn part_one(input: &str) -> i32 {
    let ((_xstart, _xend), (ystart, _yend)) = parse_input(input.trim());
    let start_yv = (ystart * -1) - 1;
    (start_yv * (start_yv + 1)) / 2
}

// total x distance for a starting xv is the sum of natural numbers up to xv: x = (xv*(xv+1))/2 = (xv^2 + xv) / 2
// solving this for a given x can be done with quadratics: 0 = 1xv^2 + 1xv - 2x
// closed form solving for xv: xv = (-1 +/-sqrt(1-4*1*-2x)) / 2*1
// reduced: xv = (-1 + sqrt(1+8x)) / 2
//
// xv: 16 - 160: 144 possible values
// yv: -142 - 141: 284 possible values
// 40896 possible combos

pub fn verify(xv: i32, yv: i32, xstart: i32, xend: i32, ystart: i32, yend: i32) -> bool {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut xv_copy = xv;
    let mut yv_copy = yv;
    while x <= xend && y >= ystart {
        x += xv_copy;
        y += yv_copy;
        if x >= xstart && x <= xend && y <= yend && y >= ystart {
            return true;
        }
        if xv_copy > 0 {
            xv_copy -= 1;
        }
        yv_copy -= 1;
    }
    false
}

pub fn part_two(input: &str) -> u32 {
    let ((xstart, xend), (ystart, yend)) = parse_input(input.trim());
    let max_xv = xend;
    let min_xv = ((((8.0 * xstart as f32) + 1.0).sqrt() - 1.0) / 2.0).ceil() as i32;
    let max_yv = (ystart * -1) - 1;
    let min_yv = ystart;
    let mut count = 0u32;
    let mut xv = min_xv;
    while xv <= max_xv {
        let mut yv = min_yv;
        while yv <= max_yv {
            if verify(xv, yv, xstart, xend, ystart, yend) {
                count += 1;
            }
            yv += 1;
        }
        xv += 1;
    }
    count
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(part_one("target area: x=20..30, y=-10..-5"), 45);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two("target area: x=20..30, y=-10..-5"), 112);
    }
}
