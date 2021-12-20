// use std::fmt::Display;

use std::fs;

#[derive(Debug)]
struct Board {
    size: usize,
    nums: Vec<usize>,
    seen: Vec<bool>,
}

impl Board {
    fn from_nums(size: usize, nums: Vec<usize>) -> Board {
        Board {
            size,
            nums,
            seen: vec![false; size * size],
        }
    }

    fn see(&mut self, num: usize) {
        for i in 0..(self.size * self.size) {
            if self.nums[i] == num {
                self.seen[i] = true
            }
        }
    }

    fn is_complete(&self) -> bool {
        for row in 0..self.size {
            let mut complete = true;

            for column in 0..self.size {
                complete &= self.seen[row * self.size + column];
            }

            if complete {
                return true;
            }
        }

        for column in 0..self.size {
            let mut complete = true;

            for row in 0..self.size {
                complete &= self.seen[column + row * self.size];
            }

            if complete {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> usize {
        let mut res = 0;

        for i in 0..(self.size * self.size) {
            if !self.seen[i] {
                res += self.nums[i]
            }
        }

        res
    }
}

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

        let b = Board::from_nums(5, nums);
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
