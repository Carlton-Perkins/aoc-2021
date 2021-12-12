use std::{collections::HashMap, io::Error};

use aoc_2021::load_data_file;
use itertools::Itertools;

// 1 -> cf
// 7 -> acf
// 4 -> bcdf
// 2 -> acdeg
// 3 -> acdfg
// 5 -> abdfg
// 0 -> abcefg
// 6 -> abdefg
// 9 -> abcdfg
// 8 -> abcdefg

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let mut acc = 0;
    for line in lines {
        let parts = line
            .split('|')
            .map(|x| {
                x.split(' ')
                    .filter(|s| *s != "")
                    .map(|s| {
                        s.chars().sorted().fold(String::new(), |mut acc, s| {
                            acc.push(s);
                            acc
                        })
                    })
                    .collect_vec()
            })
            .collect_vec();

        let left = &parts[0];
        let right = &parts[1];

        let mut mapping = HashMap::<&String, i32>::new();
        let mut rmapping = HashMap::<i32, &String>::new();
        for input in left {
            match input.len() {
                2 => {
                    mapping.insert(input, 1);
                    rmapping.insert(1, input);
                }
                7 => {
                    mapping.insert(input, 8);
                    rmapping.insert(8, input);
                }
                4 => {
                    mapping.insert(input, 4);
                    rmapping.insert(4, input);
                }
                3 => {
                    mapping.insert(input, 7);
                    rmapping.insert(7, input);
                }
                _ => {}
            }
        }

        for input in left {
            match input.len() {
                6 => {
                    // 9 -> abcdfg contains 1,4,5,7
                    if [1, 4, 7].iter().all(|x| {
                        if let Some(val) = rmapping.get(&x) {
                            val.chars().all(|c| input.contains(c))
                        } else {
                            false
                        }
                    }) {
                        mapping.insert(input, 9);
                        rmapping.insert(9, input);
                    } else
                    // 0 -> abcefg contains 1, 7
                    if [1, 7].iter().all(|x| {
                        if let Some(val) = rmapping.get(&x) {
                            val.chars().all(|c| input.contains(c))
                        } else {
                            false
                        }
                    }) {
                        mapping.insert(input, 0);
                        rmapping.insert(0, input);
                    } else {
                        // 6 -> abdefg contains 5
                        mapping.insert(input, 6);
                        rmapping.insert(6, input);
                    }
                }
                _ => {}
            }
        }

        for input in left {
            match input.len() {
                5 => {
                    // 3 -> acdfg contains 7
                    if [7].iter().all(|x| {
                        if let Some(val) = rmapping.get(&x) {
                            val.chars().all(|c| input.contains(c))
                        } else {
                            false
                        }
                    }) {
                        mapping.insert(input, 3);
                        rmapping.insert(3, input);
                    } else
                    // 5 -> abdfg contained by 6,9
                    if [6, 9].iter().all(|x| {
                        if let Some(val) = rmapping.get(&x) {
                            input.chars().all(|c| val.contains(c))
                        } else {
                            false
                        }
                    }) {
                        mapping.insert(input, 5);
                        rmapping.insert(5, input);
                    } else
                    // 2 -> acdeg
                    {
                        {
                            mapping.insert(input, 2);
                            rmapping.insert(2, input);
                        }
                    }
                }
                _ => {}
            }
        }
        println!("{:?}", mapping);
        println!("{:?}", rmapping);
        println!("{:?}", right);
        let solve = (format!(
            "{}{}{}{}",
            mapping.get(&right[0]).unwrap(),
            mapping.get(&right[1]).unwrap(),
            mapping.get(&right[2]).unwrap(),
            mapping.get(&right[3]).unwrap()
        ))
        .parse::<i32>()
        .unwrap();

        println!("{:?}", solve);
        acc += solve
    }

    println!("{}", acc);
    return Ok(());
}
