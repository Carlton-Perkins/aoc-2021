use std::io::Error;

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let mut fish_by_days = data.lines().collect_vec()[0]
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .sorted()
        .group_by(|x| *x)
        .into_iter()
        .map(|x| (x.0, x.1.count()))
        .collect_vec();

    for _ in 0..256 {
        let mut new_fbd = vec![];
        let mut new_fish = 0;
        for (day, fishes) in fish_by_days {
            match day {
                0 => {
                    new_fish += fishes;
                    new_fbd.push((8, fishes));
                }
                7 => {
                    new_fish += fishes;
                }
                n => new_fbd.push((n - 1, fishes)),
            }
        }
        new_fbd.push((6, new_fish));
        fish_by_days = new_fbd;
    }

    println!("{}", fish_by_days.iter().fold(0, |acc, x| acc + x.1));

    return Ok(());
}
