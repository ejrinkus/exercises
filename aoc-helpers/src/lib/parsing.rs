use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::IResult;
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
            if let Ok((rem, _)) = take_tag(remainder, *key) {
                if consume {
                    return (rem, Some(*val));
                }
                if let Ok((rem, _)) = take_n(remainder, 1) {
                    return (rem, Some(*val));
                } else {
                    return ("", Some(*val));
                }
            }
        }
        if inc_text {
            for (key, val) in TEXT_DIGITS.iter() {
                if let Ok((rem, _)) = take_tag(remainder, *key) {
                    if consume {
                        return (rem, Some(*val));
                    }
                    if let Ok((rem, _)) = take_n(remainder, 1) {
                        return (rem, Some(*val));
                    } else {
                        return ("", Some(*val));
                    }
                }
            }
        }
        if let Ok((rem, _)) = take_n(remainder, 1) {
            remainder = rem;
        } else {
            break;
        }
    }
    return ("", None);
}

pub fn take_tag<'a>(s: &'a str, t: &'a str) -> IResult<&'a str, &'a str> {
    tag(t)(s)
}

pub fn take_number(s: &str) -> IResult<&str, &str> {
    digit1(s)
}

pub fn take_char(s: &str) -> IResult<&str, char> {
    anychar(s)
}

pub fn take_u8(s: &str) -> IResult<&str, u8> {
    u8(s)
}

pub fn take_u16(s: &str) -> IResult<&str, u16> {
    u16(s)
}

pub fn take_u32(s: &str) -> IResult<&str, u32> {
    u32(s)
}

pub fn take_u64(s: &str) -> IResult<&str, u64> {
    u64(s)
}

pub fn take_u128(s: &str) -> IResult<&str, u128> {
    u128(s)
}

pub fn take_i8(s: &str) -> IResult<&str, i8> {
    i8(s)
}

pub fn take_i16(s: &str) -> IResult<&str, i16> {
    i16(s)
}

pub fn take_i32(s: &str) -> IResult<&str, i32> {
    i32(s)
}

pub fn take_i64(s: &str) -> IResult<&str, i64> {
    i64(s)
}

pub fn take_i128(s: &str) -> IResult<&str, i128> {
    i128(s)
}

pub fn take_spaces(s: &str) -> IResult<&str, &str> {
    space1(s)
}

pub fn take_n(s: &str, n: usize) -> IResult<&str, &str> {
    take(n)(s)
}

pub fn take_card_char(s: &str) -> IResult<&str, char> {
    one_of("AKQJT98765432")(s)
}
