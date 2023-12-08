use core::panic;
use std::collections::HashMap;

pub fn solve(input: &str) -> u64 {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut map = HashMap::new();
    while let Some(path) = lines.next() {
        let (name, choices) = path.split_once('=').unwrap();
        let (left, right) = choices[2..choices.len() - 1].split_once(", ").unwrap();

        map.insert(name.trim(), (left, right));
    }

    let positions: Vec<&str> = map.keys().filter(|&n| n.ends_with('A')).copied().collect();

    let mut counts: Vec<u64> = positions
        .iter()
        .map(|&s| calc_jumps_to_goal(s, &instructions, &map))
        .collect();

    counts.sort_unstable();
    counts.reverse();

    let largest = counts.remove(0);
    let mut result = largest;

    loop {
        if all_dividable(result, &counts) {
            break result;
        }

        result += largest;
    }
}

fn calc_jumps_to_goal(
    start: &str,
    instructions: &Vec<char>,
    map: &HashMap<&str, (&str, &str)>,
) -> u64 {
    let mut count = 0;
    let mut index = 0;

    let mut position = start;
    while !position.ends_with('Z') {
        let instruction = instructions[index];
        let (left, right) = map.get(position).unwrap();

        position = if instruction == 'L' { left } else { right };
        count += 1;
        index += 1;

        if index >= instructions.len() {
            index = 0;
        }
    }

    println!("{} jumps for {}", count, start);

    count
}

fn all_dividable(num: u64, checks: &Vec<u64>) -> bool {
    for check in checks {
        if num % check != 0 {
            return false;
        }
    }

    return true;
}
