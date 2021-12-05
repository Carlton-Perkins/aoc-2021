use std::{collections::HashMap, io::Error};

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data
        .lines()
        .map(|line| {
            line.split(|x| x == ' ' || x == ',')
                .filter_map(|n| match n.parse::<i32>() {
                    Ok(num) => Some(num),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    let mut grid = HashMap::new();
    for line in lines {
        let points = expand_line(line[0], line[1], line[2], line[3]);
        for point in points {
            if let Some(exists) = grid.clone().get(&point) {
                grid.insert(point, exists + 1);
            } else {
                grid.insert(point, 1);
            }
        }
    }

    let mut sum = 0;
    for points in grid.iter() {
        if *points.1 >= 2 {
            sum += 1;
        }
    }

    println!("sum {}", sum);

    return Ok(());
}

fn expand_line(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut v = vec![];
    let start;
    let end;
    if x1 == x2 {
        if y1 > y2 {
            start = y2;
            end = y1;
        } else {
            start = y1;
            end = y2;
        }
        for y in start..=end {
            v.push((x1, y));
        }
    } else if y1 == y2 {
        if x1 > x2 {
            start = x2;
            end = x1;
        } else {
            start = x1;
            end = x2;
        }
        for x in start..=end {
            v.push((x, y1));
        }
    } else {
        let slope_x = (x2 - x1).signum();
        let slope_y = (y2 - y1).signum();
        let mut point = (x1, y1);
        let end = (x2, y2);

        while point != end {
            v.push(point);
            point = (point.0 + slope_x, point.1 + slope_y);
        }
        v.push(point);
    }

    v
}
