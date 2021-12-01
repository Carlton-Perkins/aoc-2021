use std::io::Error;

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();
    let lines_data = lines.map(|line| line.parse::<i32>().unwrap());

    let mut acc = 0;
    let mut last = lines_data.clone().take(3).sum();
    for (a, b, c) in lines_data.tuple_windows().skip(1) {
        let val = a + b + c;
        if val > last {
            acc += 1;
        }
        last = val;
    }

    println!("{}", acc);
    return Ok(());
}
