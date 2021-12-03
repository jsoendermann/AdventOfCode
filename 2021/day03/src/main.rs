use std::fs;

const NUM_DIGITS: usize = 12;

fn main() {
    println!("Part 1: {}", part1());

    // Incomplete
    println!("Part 2: {}", part2());
}

fn get_counts() -> (usize, [usize; NUM_DIGITS]) {
    let mut bit_counts = [0; NUM_DIGITS];
    let mut total_count = 0;

    let content = fs::read_to_string("input.txt").unwrap();

    for line in content.lines() {
        for (num, character) in line.chars().enumerate() {
            if character == '1' {
                bit_counts[num] += 1;
            }
        }
        total_count += 1;
    }

    (total_count, bit_counts)
}

fn get_power_consumption() -> (usize, usize) {
    let (total_count, bit_counts) = get_counts();

    let mut gamma = 0;
    for bit_count in bit_counts.iter() {
        gamma <<= 1;
        if *bit_count >= total_count / 2 {
            gamma |= 1;
        }
    }

    (gamma, (!gamma & (usize::pow(2, 12) - 1)))
}

fn part1() -> usize {
    let (gamma, epsilon) = get_power_consumption();

    gamma * epsilon
}

fn part2() -> usize {
    let (gamma, epsilon) = get_power_consumption();
    let oxygen_rating = filter_candidates(|bit_count, total_count, has_bit_set| {
        (bit_count >= total_count / 2) == has_bit_set
    });
    let co2_rating = filter_candidates(|bit_count, total_count, has_bit_set| {
        (bit_count <= total_count / 2) == !has_bit_set
    });

    oxygen_rating * co2_rating
}

fn filter_candidates(predicate: fn(usize, usize, bool) -> bool) -> usize {
    let (total_count, bit_counts) = get_counts();

    let content = fs::read_to_string("input.txt").unwrap();

    let mut candidates = content
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect::<Vec<usize>>();

    println!("{:#?}", candidates);

    for digit in 0..NUM_DIGITS {
        let mask = 1 << (12 - digit - 1);
        candidates = candidates
            .into_iter()
            .filter(|n| {
                let has_bit_set = (mask & n) > 0;

                println!(
                    "n: {:b}, has: {}, should: {}",
                    n,
                    has_bit_set,
                    bit_counts[digit] >= total_count / 2
                );
                let res = predicate(bit_counts[digit], total_count, has_bit_set);
                println!("res: {}", res);
                res
            })
            .collect();
        if candidates.len() == 1 {
            return candidates[0];
        }

        println!("{:#?}", candidates);
    }

    panic!("No candidate found")
}
