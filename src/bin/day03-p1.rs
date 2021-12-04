use std::io::Error;

use aoc_2021::load_data_file;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let mut line_count = 0;
    let mut line_values: [i32; 12] = [0; 12];
    for line in lines {
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

    println!("{:?}", line_values);
    let mut g_acc = String::new();
    let mut e_acc = String::new();

    for line_value in line_values.iter() {
        if *line_value <= (line_count / 2) {
            g_acc.push('0');
            e_acc.push('1');
        } else {
            g_acc.push('1');
            e_acc.push('0');
        }
    }
    let g_val = isize::from_str_radix(&g_acc, 2).unwrap();
    let e_val = isize::from_str_radix(&e_acc, 2).unwrap();

    println!("{}", g_val * e_val);
    return Ok(());
}
