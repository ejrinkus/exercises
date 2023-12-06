use nom::bytes::complete::*;
use nom::error::Error;
use std::collections::HashMap;

lazy_static! {
    static ref DIGITS: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("1", 1u8);
        m.insert("2", 2u8);
        m.insert("3", 3u8);
        m.insert("4", 4u8);
        m.insert("5", 5u8);
        m.insert("6", 6u8);
        m.insert("7", 7u8);
        m.insert("8", 8u8);
        m.insert("9", 9u8);
        m.insert("0", 0u8);
        m
    };
    static ref TEXT_DIGITS: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("one", 1u8);
        m.insert("two", 2u8);
        m.insert("three", 3u8);
        m.insert("four", 4u8);
        m.insert("five", 5u8);
        m.insert("six", 6u8);
        m.insert("seven", 7u8);
        m.insert("eight", 8u8);
        m.insert("nine", 9u8);
        m.insert("zero", 0u8);
        m
    };
}

pub fn parse_matrix(s: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(s.lines().count());
    for l in s.lines() {
        let row: Vec<char> = l.chars().collect();
        matrix.push(row);
    }
    matrix
}

pub fn get_next_digit(s: &str, inc_text: bool, consume: bool) -> (&str, Option<u8>) {
    let mut remainder = s;
    while remainder != "" {
        for (key, val) in DIGITS.iter() {
            if let Ok((rem, _)) = tag::<_, _, Error<_>>(*key)(remainder) {
                if consume {
                    return (rem, Some(*val));
                }
                if let Ok((rem, _)) = take::<_, _, Error<_>>(1usize)(remainder) {
                    return (rem, Some(*val));
                } else {
                    return ("", Some(*val));
                }
            }
        }
        if inc_text {
            for (key, val) in TEXT_DIGITS.iter() {
                if let Ok((rem, _)) = tag::<_, _, Error<_>>(*key)(remainder) {
                    if consume {
                        return (rem, Some(*val));
                    }
                    if let Ok((rem, _)) = take::<_, _, Error<_>>(1usize)(remainder) {
                        return (rem, Some(*val));
                    } else {
                        return ("", Some(*val));
                    }
                }
            }
        }
        if let Ok((rem, _)) = take::<_, _, Error<_>>(1usize)(remainder) {
            remainder = rem;
        } else {
            break;
        }
    }
    return ("", None);
}
