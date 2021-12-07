use std::io::Error;

use aoc_2021::load_data_file;
use itertools::{Itertools, MinMaxResult};

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines().collect_vec()[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();

    let mut low_cost = i32::MAX;
    let mut low_n = 0;
    if let MinMaxResult::MinMax(min, max) = lines.clone().iter().minmax() {
        for n in *min..=*max {
            let cost = get_cost(&lines, n);
            if cost < low_cost {
                low_cost = cost;
                low_n = n;
            }
        }
    }

    println!("{} -> {}", low_n, low_cost);

    return Ok(());
}

fn get_cost(crabs: &Vec<i32>, loc: i32) -> i32 {
    crabs
        .iter()
        .fold(0, |acc, crab| acc + get_fact((loc - crab).abs()))
}

fn get_fact(n: i32) -> i32 {
    (1..=n).into_iter().sum()
}
