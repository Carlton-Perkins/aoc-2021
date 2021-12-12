use std::{collections::VecDeque, io::Error};

use aoc_2021::load_data_file;

enum Chunk {
    Left(char),
}

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let mut tscore_acc = 0;
    for line in lines {
        let mut score_acc = 0;
        let mut stack = VecDeque::new();
        for c in line.chars().enumerate() {
            match c.1 {
                l @ ('(' | '[' | '{' | '<') => stack.push_back(Chunk::Left(l)),
                r @ (')' | ']' | '}' | '>') => match stack.pop_back().unwrap() {
                    Chunk::Left(lv) => {
                        if val(r) != val(lv) {
                            println!("Failed on {} r != lv {} {}", c.0, r, &lv);
                            score_acc = score(r);
                            break;
                        }
                    }
                },
                _ => panic!("unsupported char {}", &c.1),
            }
        }
        tscore_acc += score_acc;
        println!("{:?} -> {}", line, score_acc)
    }

    println!("{}", tscore_acc);

    return Ok(());
}

fn score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
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
