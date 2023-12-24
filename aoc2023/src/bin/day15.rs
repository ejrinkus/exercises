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
        15
    }

    fn part_one(&self, input: &str) -> String {
        let mut sum = 0;
        let chunks = input.split(",");
        for chunk in chunks {
            sum += run_hash(chunk);
        }
        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut boxes: Vec<Vec<(&str, usize)>> = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Vec::new());
        }

        let chunks = input.split(",");
        for chunk in chunks {
            let (label, (c, lens)) = parse_chunk(chunk);
            let hash = run_hash(label);
            if c == '-' {
                let b = boxes.get_mut(hash as usize).unwrap();
                let mut i = 0;
                for (blabel, _) in b.iter() {
                    if &label == blabel {
                        break;
                    }
                    i += 1;
                }
                if i < b.len() {
                    b.remove(i);
                }
            } else if c == '=' {
                let b = boxes.get_mut(hash as usize).unwrap();
                let mut i = 0;
                for (blabel, _) in b.iter() {
                    if &label == blabel {
                        break;
                    }
                    i += 1;
                }
                if i < b.len() {
                    b.get_mut(i).unwrap().1 = lens;
                } else {
                    b.push((label, lens));
                }
            }
        }

        let mut sum = 0;
        for i in 0..boxes.len() {
            for j in 0..boxes[i].len() {
                sum += (1 + i) * (1 + j) * boxes[i][j].1;
            }
        }
        sum.to_string()
    }
}

fn run_hash(s: &str) -> u32 {
    let mut val = 0;
    for c in s.chars() {
        if c == ',' || c == '\n' {
            continue;
        }
        val += c as u32;
        val *= 17;
        val %= 256;
    }
    val
}

fn parse_chunk(s: &str) -> (&str, (char, usize)) {
    if &s[(s.len() - 1)..] == "-" {
        let label = &s[..(s.len() - 1)];
        return (label, ('-', 0));
    }
    let idx = s.rfind('=').unwrap();
    let (label, equals) = s.split_at(idx);
    let lens = equals[1..].trim();
    return (label, ('=', lens.parse::<usize>().unwrap()));
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            "1320"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            "145"
        );
    }
}
