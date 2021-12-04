use std::io::Error;

use aoc_2021::load_data_file;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let data = load_data_file()?;

    let lines = data.lines();

    let input = lines.clone().take(2).collect_vec()[0]
        .split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect_vec();

    let mut boards = vec![];
    let mut current_board = vec![];
    for line in lines.skip(2) {
        println!("{}", line);
        if line == "" {
            println!("new board");
            boards.push(current_board);
            current_board = vec![];
        } else {
            let mut nums = line
                .split_whitespace()
                .map(|x| x.parse::<u16>().unwrap())
                .collect_vec();

            current_board.append(&mut nums)
        }
    }
    boards.push(current_board);

    let mut called = vec![];
    let mut boards_won = vec![];
    for call in input {
        println!("adding number {}", call);
        called.push(call.clone());

        for board in boards.clone().iter().enumerate() {
            if let Some(a) = get_solved(&board.1, &called) {
                println!("BOARD {} SOLVED {}", board.0, a);
                if !boards_won.contains(&board.0) {
                    boards_won.push(board.0)
                }

                if boards_won.len() == boards.len() {
                    return Ok(());
                }
            }
        }
    }

    return Ok(());
}

fn get_solved(board: &Vec<u16>, called_nums: &Vec<u16>) -> Option<u64> {
    if board.chunks(5).any(|x| {
        println!("{:?}", x);
        x.iter().all(|f| called_nums.contains(f))
    }) {
        println!(
            "solved board {:?} with {:?}",
            board,
            called_nums
                .iter()
                .filter(|x| board.contains(x))
                .collect_vec()
        );
        return Some(
            board
                .iter()
                .filter(|&x| !called_nums.contains(x))
                .cloned()
                .fold(0, |acc, e| acc + e as u64)
                * *called_nums.last().unwrap() as u64,
        );
    } else {
        println!("new vert");
        for col in 0..5 {
            let mut found = 0;
            println!("new vert col");
            for n in 0..5 {
                println!("vert check {}", board[5 * n + col]);
                if called_nums.contains(&board[5 * n + col]) {
                    found += 1;
                }
                if found == 5 {
                    println!(
                        "solved board {:?} with {:?}",
                        board,
                        called_nums
                            .iter()
                            .filter(|x| board.contains(x))
                            .collect_vec()
                    );
                    return Some(
                        board
                            .iter()
                            .filter(|&x| !called_nums.contains(x))
                            .cloned()
                            .fold(0, |acc, e| acc + e as u64)
                            * *called_nums.last().unwrap() as u64,
                    );
                }
            }
        }

        return None;
    }
}
