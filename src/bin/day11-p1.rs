use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use aoc_2021::load_data_file;
use itertools::Itertools;

type Pos = (isize, isize);
type Map = HashMap<Pos, i32>;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x as isize, y as isize),
                        c.to_string().parse::<i32>().unwrap(),
                    )
                })
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let mut map = HashMap::new();
    for (pos, v) in lines {
        map.insert(pos, v);
    }

    let mut flashed = 0;
    for n in 0..100 {
        println!("2,2 -> {:?}", map.get(&(2, 2)).unwrap());
        let (flashes, nmap) = sim(map);
        map = nmap;
        flashed += flashes;

        println!("{} -> {}", n, flashes);
    }

    println!("{}", flashed);

    return Ok(());
}

fn sim(map: Map) -> (i32, Map) {
    // Add 1 to all
    let mut new_map: Map = map.iter().map(|(p, v)| (*p, v + 1)).collect();

    // Flash cascade
    let mut flashed = HashSet::new();
    let mut last_flashed_len = 0;
    loop {
        let mut updates = vec![];
        for (pos, v) in new_map.iter() {
            if *v <= 9 {
                continue;
            }

            if flashed.contains(pos) {
                continue;
            }
            flashed.insert(*pos);
            println!("flash on {:?}", &pos);

            let adjm = vec![
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 - 1),
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 1, pos.1 - 1),
                (pos.0 - 1, pos.1 + 1),
            ];

            for adj in adjm {
                updates.push(adj);
            }
        }

        for u in updates {
            if let Some(v) = new_map.get_mut(&u) {
                *v += 1;
            }
        }

        if flashed.len() == last_flashed_len {
            break;
        }
        last_flashed_len = flashed.len()
    }

    new_map = new_map
        .iter()
        .map(|(p, v)| (*p, if v > &9 { 0 } else { *v }))
        .collect();

    (flashed.len() as i32, new_map)
}
