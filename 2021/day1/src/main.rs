use std::fs;

fn main() {
    let res = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();
    println!("Part 1: {}", res);

    let res = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();

    println!("Part 2: {}", res);
}
