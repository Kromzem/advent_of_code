use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    input.lines().map(calc_worth).sum()
}

fn calc_worth(line: &str) -> u32 {
    let (_, game) = line.split_once(':').expect("Should be splittable");

    let (winning, users) = game.split_once('|').expect("Two game sites");

    let winning_numbers: HashSet<u32> = HashSet::from_iter(get_numbers(winning));
    let user_numbers = get_numbers(users);

    let count = user_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();

    if count == 0 {
        0
    } else {
        u32::pow(2, (count as u32) - 1)
    }
}

fn get_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .trim()
        .split_ascii_whitespace()
        .map(|n| u32::from_str_radix(n, 10).expect("Number parsable"))
        .collect()
}
