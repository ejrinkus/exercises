use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2020
    }
    fn day(&self) -> u32 {
        18
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn parse_expr(line: &str) -> i64 {
    let mut depth = 0_usize;
    // Pairs of totals and last-operator-seen for each depth.
    let mut depth_totals: Vec<(i64, char)> = Vec::new();
    depth_totals.push((0, '+'));
    for c in line.chars() {
        if c == ' ' {
            continue;
        }
        let mut t_and_o = depth_totals.get_mut(depth).unwrap();
        if let Ok(val) = c.to_string().parse::<i64>() {
            match t_and_o.1 {
                '+' => (*t_and_o).0 += val,
                '*' => (*t_and_o).0 *= val,
                _ => panic!("unexpected operator {}", t_and_o.1),
            };
            continue;
        }
        match c {
            '+' => {
                (*t_and_o).1 = '+';
            }
            '*' => {
                (*t_and_o).1 = '*';
            }
            '(' => {
                depth += 1;
                if depth >= depth_totals.len() {
                    depth_totals.push((0, '+'));
                } else {
                    depth_totals[depth] = (0, '+');
                }
            }
            ')' => {
                depth -= 1;
                let val = (*t_and_o).0;
                let mut next_t_and_o = depth_totals.get_mut(depth).unwrap();
                match next_t_and_o.1 {
                    '+' => (*next_t_and_o).0 += val,
                    '*' => (*next_t_and_o).0 *= val,
                    _ => panic!("unexpected operator {}", next_t_and_o.1),
                };
            }
            _ => panic!("unexpected operator {}", c),
        }
    }
    depth_totals[0].0
}

pub fn surround_addition(line: &mut String) {
    let mut depth = 0;
    let mut open_depth: Vec<bool> = Vec::new();
    open_depth.push(false);
    let mut i = 0;
    while i < line.len() {
        let curr = *line.as_bytes().get(i).unwrap() as char;
        match curr {
            d if d.is_numeric() => {}
            ' ' => {}
            '+' => {
                if !open_depth[depth] {
                    // if the previous character was a ')', we need to scan back to the
                    // matching '('.
                    if *line.as_bytes().get(i - 2).unwrap() as char == ')' {
                        let s = line
                            .chars()
                            .rev()
                            .skip(line.len() - i + 1)
                            .scan((0, i), |state, c| {
                                if c == ')' {
                                    (*state).0 += 1;
                                } else if c == '(' {
                                    (*state).0 -= 1;
                                }
                                state.1 -= 1;
                                Some(*state)
                            })
                            .skip_while(|s| s.0 > 0)
                            .next()
                            .unwrap();
                        line.insert(s.1, '(');
                    } else {
                        line.insert(i - 2, '(');
                    }
                    i += 1;
                    depth += 1;
                    if depth >= open_depth.len() {
                        open_depth.push(true);
                    } else {
                        open_depth[depth] = true;
                    }
                }
            }
            '*' => {
                if open_depth[depth] {
                    line.insert(i - 1, ')');
                    i += 1;
                    open_depth[depth] = false;
                    depth -= 1;
                }
            }
            '(' => {
                depth += 1;
                if depth >= open_depth.len() {
                    open_depth.push(false);
                } else {
                    open_depth[depth] = false;
                }
            }
            ')' => {
                if open_depth[depth] {
                    line.insert(i, ')');
                    i += 1;
                    open_depth[depth] = false;
                    depth -= 1;
                }
                depth -= 1;
            }
            _ => panic!("unexpected character {}", curr),
        }
        i += 1;
    }
    while depth != 0 {
        if open_depth[depth] {
            line.push(')');
        }
        depth -= 1;
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        sum += parse_expr(line);
    }
    sum
}

pub fn part_two(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut owned = line.to_owned();
        surround_addition(&mut owned);
        let result = parse_expr(&owned);
        sum += result;
    }
    sum
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        assert_eq!(part_one("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_one("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part_one("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part_two("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part_two("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            part_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            part_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
