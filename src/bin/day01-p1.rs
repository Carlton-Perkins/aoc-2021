use std::io::Error;

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let mut acc = 0;
    let mut last = lines.clone().take(1).collect_vec()[0]
        .parse::<i32>()
        .unwrap();
    for line in lines.skip(1) {
        let val = line.parse::<i32>().unwrap();
        if val > last {
            acc += 1;
        }
        last = val;
    }

    println!("{}", acc);
    return Ok(());
}
