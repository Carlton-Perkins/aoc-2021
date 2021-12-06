use std::io::Error;

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let mut fish = data.lines().collect_vec()[0]
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect_vec();

    for _ in 0..80 {
        let mut new_fish = vec![];
        fish = fish
            .iter()
            .map(|fish| match *fish {
                0 => {
                    new_fish.push(8);
                    6
                }
                n => n - 1,
            })
            .collect();
        fish.append(&mut new_fish);
    }

    println!("{}", fish.len());

    return Ok(());
}
