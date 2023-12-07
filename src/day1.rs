use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!("142", "281");

fn process_line_a(line: &String) -> usize {
    let lchar = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();
    let rchar = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();
    (lchar * 10 + rchar) as usize
}

pub fn process_a(lines: Vec<String>) -> String {
    lines.iter().map(process_line_a).sum::<usize>().to_string()
}

static WORDS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_digit(line: &str) -> Option<u32> {
    if line.chars().next().unwrap().is_ascii_digit() {
        return line.chars().next().unwrap().to_digit(10);
    }
    WORDS
        .iter()
        .find(|(word, _digit)| line.starts_with(word))
        .map(|(_word, digit)| *digit)
}

fn process_line_b(line: &String) -> usize {
    let lidx = (0..line.len())
        .find(|idx| parse_digit(&line[*idx..]).is_some())
        .unwrap();
    let ridx = (0..line.len())
        .rev()
        .find(|idx| parse_digit(&line[*idx..]).is_some())
        .unwrap();
    let lchar = parse_digit(&line[lidx..]).unwrap();
    let rchar = parse_digit(&line[ridx..]).unwrap();
    (lchar * 10 + rchar) as usize
}

pub fn process_b(lines: Vec<String>) -> String {
    lines.iter().map(process_line_b).sum::<usize>().to_string()
}
