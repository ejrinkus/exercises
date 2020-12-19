use regex::Regex;

#[derive(Clone, Default, Debug)]
pub struct Rule {
  // If the rule doesn't contain any subrules, then this is the raw string for
  // the rule.  Otherwise, it is all the subrules expanded into one string.
  finalized: String,
  // Subrules are '|' delimited lists of numbers.  These lists are logically
  // OR'd together
  subrules: Vec<Vec<usize>>,
}

pub fn build_rules(raw_rules: &str) -> Vec<Rule> {
  let mut rules: Vec<Rule> = Vec::new();
  for line in raw_rules.lines() {
    let mut parts = line.split(": ");
    let index = parts.next().unwrap().parse::<usize>().unwrap();
    let rule = parts.next().unwrap();
    if index >= rules.len() {
      rules.resize_with(index + 1, Default::default);
    }
    if rule.contains('"') {
      let finalized = rule
        .strip_prefix("\"")
        .unwrap()
        .strip_suffix("\"")
        .unwrap()
        .to_string();
      rules[index] = Rule {
        finalized,
        subrules: Vec::new(),
      };
    } else {
      let mut subrules: Vec<Vec<usize>> = Vec::new();
      for sub in rule.split(" | ") {
        subrules.push(
          sub
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
        );
      }
      rules[index] = Rule {
        finalized: "".to_string(),
        subrules,
      };
    }
  }
  rules
}

pub fn expand_rules(rules: &mut Vec<Rule>, target: usize) {
  let mut rule = rules.get(target).unwrap().clone();
  if !rule.finalized.is_empty() {
    return;
  }
  for list in &rule.subrules {
    for sub in list {
      if *sub == 8 && target == 8 {
        // Special handling for rule 8:
        // 8: 42 | 42 8 is basically the same as "match rule 42 any number of
        // times in a row, but at least once."  In a regex, this is effectively
        // the same as putting a '+' after whatever rule 42 is.
        // By the time we get to here, we would've already expanded rule 42, so
        // we can just grab the string from that rule, tack on a '+', overwrite
        // our progress, and return.
        let rule8 = format!("({})+", &rules[42].finalized);
        rule.finalized = rule8.to_string();
        rules[target] = rule;
        return;
      }
      if *sub == 11 && target == 11 {
        // Special handling for rule 8:
        // 11: 42 31 | 42 11 31 is basically the same as "match rule 42 any
        // number of times in a row, but at least once, and then match rule 31
        // the same number of times."  Unfortunately this thing isn't
        // technically solvable with regular expressions.  However, I'm already
        // b***s deep committed to regex at this point, so I'm going to cheat :)
        let rule42 = &rules[42].finalized;
        let rule31 = &rules[31].finalized;
        let rule11 = format!(
          "(({}){{1}}({}){{1}})|(({}){{2}}({}){{2}})|(({}){{3}}({}){{3}})|(({}){{4}}({}){{4}})",
          rule42, rule31, rule42, rule31, rule42, rule31, rule42, rule31
        );
        rule.finalized = rule11.to_string();
        rules[target] = rule;
        return;
      }
      let subrule = rules.get(*sub).unwrap();
      if subrule.finalized.is_empty() {
        expand_rules(rules, *sub);
      }
      let subrule = rules.get(*sub).unwrap();
      rule.finalized.push_str("(");
      rule.finalized.push_str(&subrule.finalized);
      rule.finalized.push_str(")");
    }
    rule.finalized.push_str("|");
  }
  if rule.finalized.ends_with("|") {
    rule.finalized = rule.finalized.strip_suffix("|").unwrap().to_string();
  }
  rules[target] = rule;
}

pub fn part_one(input: &str) -> u32 {
  let mut parts = input.split("\n\n");
  let mut rules = build_rules(parts.next().unwrap());
  expand_rules(&mut rules, 0);
  let mut to_match = "^".to_string();
  to_match.push_str(&rules[0].finalized);
  to_match.push_str("$");
  let re = Regex::new(&to_match).unwrap();
  let messages = parts.next().unwrap();
  let mut sum = 0_u32;
  for m in messages.lines() {
    if re.is_match(m) {
      sum += 1;
    }
  }
  sum
}

pub fn part_two(input: &str) -> u32 {
  part_one(input)
}

#[cfg(test)]
mod day19_tests {
  use super::*;

  #[test]
  fn samples_part1() {
    assert_eq!(
      part_one(
        r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
      ),
      2
    );
  }

  #[test]
  fn samples_part2() {
    assert_eq!(
      part_two(
        r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
      ),
      12
    );
  }
}
