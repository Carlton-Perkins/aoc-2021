use std::{collections::VecDeque, io::Error};

use aoc_2021::load_data_file;
use itertools::Itertools;

#[derive(Debug)]
enum Chunk {
    Left(char),
}

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let mut scores = vec![];
    for line in lines {
        if let Some(s) = fix(line) {
            println!("line {} -> fix {:?}", &line, &s);
            scores.push(s);
        }
    }

    let fscores: Vec<i64> = scores
        .iter()
        .map(|l| l.iter().map(score).fold(0, |acc, n| (acc * 5) + n))
        .sorted()
        .collect();
    println!("{}", fscores[fscores.len() / 2]);

    return Ok(());
}

fn fix(line: &str) -> Option<Vec<char>> {
    let mut stack = VecDeque::new();
    for c in line.chars().enumerate() {
        match c.1 {
            l @ ('(' | '[' | '{' | '<') => stack.push_back(Chunk::Left(l)),
            r @ (')' | ']' | '}' | '>') => match stack.pop_back().unwrap() {
                Chunk::Left(lv) => {
                    if val(r) != val(lv) {
                        return None;
                    }
                }
            },
            _ => panic!("unsupported char {}", &c.1),
        }
    }

    println!("Fixing line {}", &line);

    return Some(stack.iter().rev().map(invert).collect());
}

fn score(c: &char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unsupported char {}", &c),
    }
}

fn val(c: char) -> u8 {
    match c {
        '(' | ')' => 1,
        '[' | ']' => 2,
        '{' | '}' => 3,
        '<' | '>' => 4,
        _ => panic!("unsupported char {}", &c),
    }
}

fn invert(ch: &Chunk) -> char {
    let Chunk::Left(c) = ch;
    return match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unsupported char {}", &c),
    };
}
