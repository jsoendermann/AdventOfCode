// use std::fmt::Display;

use std::fs;

mod board;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines().peekable();

    let input = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    lines.next();

    let mut boards = Vec::new();

    while lines.peek().is_some() {
        let mut nums = Vec::new();

        for _ in 0..5 {
            let next_line = lines.next().unwrap();
            next_line
                .split_whitespace()
                .for_each(|s| nums.push(s.parse::<usize>().unwrap()));
        }

        let b = board::Board::from_nums(5, nums);
        boards.push(b);

        // println!("{:?}", b);

        lines.next();
    }

    let mut last_winner_score = 0;
    for num in input {
        println!("Input: {}", num);
        for board in &mut boards {
            if board.is_complete() {
                continue;
            }
            board.see(num);

            if board.is_complete() {
                println!("Winner!");
                last_winner_score = board.unmarked_sum() * num;
            }
        }
    }

    println!("Last winner score: {}", last_winner_score);

    // println!("{:?}", input);
}
