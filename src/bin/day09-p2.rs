use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let hmap = data
        .lines()
        .enumerate()
        .map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x as isize, y as isize),
                        c.to_string().parse::<i32>().unwrap(),
                    )
                })
                .collect_vec()
        })
        .fold(Vec::new(), |acc, n| [acc, n].concat());

    let mut map = HashMap::new();
    for (pos, v) in &hmap {
        map.insert(pos, v);
    }

    let mut res = HashMap::new();
    for (pos, v) in &hmap {
        let low = find_low(pos, &map);
        if low {
            res.insert(pos, v);
        }
    }

    let mut acc = vec![];
    for low in res.keys() {
        let v = find_basin_size(low, &map);
        println!("{:?} -> {}", low, v);
        acc.push(v);
    }

    println!(
        "{}",
        acc.iter().sorted().rev().take(3).fold(1, |acc, n| acc * n)
    );
    // println!("{:?}", &res);
    // println!(
    //     "{:?}",
    //     res.values().into_iter().fold(0, |acc, n| acc + **n + 1)
    // );

    return Ok(());
}

type Pos = (isize, isize);

fn find_low(pos: &Pos, map: &HashMap<&Pos, &i32>) -> bool {
    let adjm = vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ];

    let current = map.get(pos).unwrap();
    for adj in adjm {
        if let Some(h) = map.get(&adj) {
            if h <= current {
                return false;
            }
        }
    }
    return true;
}

fn find_basin_size(pos: &Pos, map: &HashMap<&Pos, &i32>) -> i32 {
    println!("new explore {:?}", &pos);
    let mut explore = vec![*pos];
    let mut explored = HashSet::new();
    while explore.len() != 0 {
        let e = explore.pop().unwrap();
        println!("exploring {:?}", e);
        explored.insert(e);
        let adjm = vec![
            (e.0 - 1, e.1),
            (e.0 + 1, e.1),
            (e.0, e.1 - 1),
            (e.0, e.1 + 1),
        ];

        for adj in adjm {
            if let Some(v) = map.get(&adj) {
                if **v != 9 && !explored.contains(&adj) {
                    println!("discovered {:?} - {}", &adj, v);
                    explore.push(adj)
                }
            }
        }
    }

    explored.len() as i32
}
