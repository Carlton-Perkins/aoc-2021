use std::{collections::HashMap, io::Error};

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
    // for (pos, _) in &hmap {
    //     let low = find_low(pos, &map);
    //     res.insert(low, map.get(&low).unwrap());
    // }

    println!("{:?}", &res);
    println!(
        "{:?}",
        res.values().into_iter().fold(0, |acc, n| acc + **n + 1)
    );

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

// fn find_low(pos: &Pos, map: &HashMap<&Pos, &i32>) -> Pos {
//     let adjm = vec![
//         (pos.0 - 1, pos.1),
//         (pos.0 + 1, pos.1),
//         (pos.0, pos.1 - 1),
//         (pos.0, pos.1 + 1),
//     ];

//     let mut current = map.get(pos).unwrap();
//     let mut min = *pos;
//     for adj in adjm {
//         if let Some(h) = map.get(&adj) {
//             if h < current {
//                 current = h;
//                 min = adj;
//             }
//         }
//     }
//     if min == *pos {
//         return min;
//     } else {
//         min = find_low(&min, &map);
//     }

//     min
// }
