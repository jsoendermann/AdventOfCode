use std::fs;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut depth = 0;
    let mut horizontal = 0;
    for line in content.lines() {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let distance = split.next().unwrap().parse::<usize>().unwrap();
        match direction {
            "forward" => horizontal += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => panic!("Can't parse line"),
        }
    }

    depth * horizontal
}

fn part2() -> usize {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;
    for line in content.lines() {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let value = split.next().unwrap().parse::<usize>().unwrap();
        match direction {
            "down" => aim += value,
            "up" => aim -= value,
            "forward" => {
                horizontal += value;
                depth += value * aim
            }
            _ => panic!("Can't parse line"),
        }
    }

    depth * horizontal
}
