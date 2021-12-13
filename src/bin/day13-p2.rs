use std::{collections::HashSet, io::Error};

use aoc_2021::load_data_file;
use itertools::Itertools;

type Pos = (i32, i32);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Fold {
    Y(i32),
    X(i32),
}

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let mut dots = HashSet::new();
    let mut folds = vec![];
    let mut nl = false;
    for line in lines {
        if line == "" {
            nl = true;
            continue;
        }
        if !nl {
            let s: Pos = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            dots.insert(s);
        } else {
            let s = line.split(' ').skip(2).at_most_one().unwrap().unwrap();
            folds.push(match s.split('=').collect_tuple().unwrap() {
                ("x", n) => Fold::X(n.to_string().parse::<i32>().unwrap()),
                ("y", n) => Fold::Y(n.to_string().parse::<i32>().unwrap()),
                _ => panic!(),
            });
        }
    }

    println!("dots {:?}", dots);
    println!("folds {:?}", folds);

    for f in folds.iter() {
        dots = fold(f, dots)
    }

    let mut mmx = i32::MIN;
    let mut mmy = i32::MIN;
    for p in dots.iter() {
        mmx = mmx.max(p.0);
        mmy = mmy.max(p.1);
    }

    println!("{} {}", mmx, mmy);

    for y in 0..=mmy {
        for x in 0..=mmx {
            if dots.contains(&(x, y)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("");
    }

    // println!("dots {:?} {:?}", x, y);

    return Ok(());
}

fn fold(f: &Fold, grid: HashSet<Pos>) -> HashSet<Pos> {
    let mut ngrid = HashSet::new();
    match f {
        Fold::Y(y) => {
            for p in grid {
                if p.1 < *y {
                    ngrid.insert(p);
                } else {
                    ngrid.insert((p.0, y - (p.1 - y)));
                }
            }
        }
        Fold::X(x) => {
            for p in grid {
                if p.0 < *x {
                    ngrid.insert(p);
                } else {
                    ngrid.insert((x - (p.0 - x), p.1));
                }
            }
        }
    }

    ngrid
}
