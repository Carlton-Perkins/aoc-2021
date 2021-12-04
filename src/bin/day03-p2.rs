use std::io::Error;

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let size = lines.clone().take(1).collect_vec()[0].len();

    let mut lines_vec = lines.clone().collect_vec();
    let mut bit = 0;
    while lines_vec.len() != 1 {
        println!("new run");
        let mut line_count = 0;
        let mut line_values = vec![0; size];
        for line in lines_vec.clone() {
            line_count += 1;
            for c in line.chars().enumerate() {
                match c.1 {
                    '1' => {
                        line_values[c.0] += 1;
                    }
                    _ => {}
                }
            }
        }
        println!("Line count : {}", line_count);
        println!("line values : {:?}", line_values);
        println!("remaining : {:?}", lines_vec);
        let e_bit = if line_values[bit] >= line_count - line_values[bit] {
            '0'
        } else {
            '1'
        };
        println!("Taking {}", e_bit);

        lines_vec = lines_vec
            .into_iter()
            .filter(|&x| {
                if e_bit == x.chars().nth(bit).unwrap() {
                    true
                } else {
                    false
                }
            })
            .collect();

        bit += 1;
    }

    println!("picking: {}", lines_vec[0]);
    println!("{:?}", isize::from_str_radix(lines_vec[0], 2).unwrap());

    return Ok(());
}
