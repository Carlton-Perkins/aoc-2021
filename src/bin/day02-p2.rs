use std::io::Error;

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines: Vec<(&str, i32)> = data
        .lines()
        .map(|line| {
            let parts = line.split(" ").take(2).collect_vec();
            (parts[0], parts[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut depth = 0;
    let mut hpos = 0;
    let mut aim = 0;
    for (command, num) in lines {
        match command {
            "forward" => {
                hpos += num;
                depth += aim * num;
            }
            "down" => {
                aim += num;
            }
            "up" => {
                aim -= num;
            }
            _ => {
                panic!("Unknown command: {}", command);
            }
        }
    }

    println!("{}", hpos * depth);

    return Ok(());
}
