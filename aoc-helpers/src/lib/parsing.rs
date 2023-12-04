pub fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(input.lines().count());
    for l in input.lines() {
        let row: Vec<char> = l.chars().collect();
        matrix.push(row);
    }
    matrix
}

pub fn text_to_digit(input: &str, i: usize) -> Option<u8> {
    if i + 3 <= input.len() && input.get(i..i + 3) == Some("one") {
        return Some(1);
    } else if i + 3 <= input.len() && input.get(i..i + 3) == Some("two") {
        return Some(2);
    } else if i + 5 <= input.len() && input.get(i..i + 5) == Some("three") {
        return Some(3);
    } else if i + 4 <= input.len() && input.get(i..i + 4) == Some("four") {
        return Some(4);
    } else if i + 4 <= input.len() && input.get(i..i + 4) == Some("five") {
        return Some(5);
    } else if i + 3 <= input.len() && input.get(i..i + 3) == Some("six") {
        return Some(6);
    } else if i + 5 <= input.len() && input.get(i..i + 5) == Some("seven") {
        return Some(7);
    } else if i + 5 <= input.len() && input.get(i..i + 5) == Some("eight") {
        return Some(8);
    } else if i + 4 <= input.len() && input.get(i..i + 4) == Some("nine") {
        return Some(9);
    } else if i + 4 <= input.len() && input.get(i..i + 4) == Some("zero") {
        return Some(0);
    }
    None
}
