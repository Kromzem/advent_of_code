use std::collections::HashMap;

use crate::{Puzzle, PuzzleInput};

pub struct Day11 {}

impl Puzzle<Input> for Day11 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        11
    }

    fn new() -> Self {
        Day11 {}
    }

    fn solve_part_one(&self, input: &Input) -> i64 {
        let mut cache = HashMap::new();
        input
            .stones
            .iter()
            .map(|&s| solve(s, 25, &mut cache))
            .sum::<u64>() as i64
    }

    fn solve_part_two(&self, input: &Input) -> i64 {
        let mut cache = HashMap::new();
        input
            .stones
            .iter()
            .map(|&s| solve(s, 75, &mut cache))
            .sum::<u64>() as i64
    }
}

fn solve(num: u64, remaining_depth: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if remaining_depth == 0 {
        return 1;
    }

    match cache.get(&(num, remaining_depth)) {
        Some(count) => *count,
        None => {
            let count = if num == 0 {
                solve(1, remaining_depth - 1, cache)
            } else {
                let str_num = num.to_string();
                if str_num.len() % 2 != 0 {
                    solve(num * 2024, remaining_depth - 1, cache)
                } else {
                    let (first, second) = str_num.split_at(str_num.len() / 2);
                    solve(
                        first.parse().expect("Split not a number"),
                        remaining_depth - 1,
                        cache,
                    ) + solve(
                        second.parse().expect("Split not a number"),
                        remaining_depth - 1,
                        cache,
                    )
                }
            };

            cache.insert((num, remaining_depth), count);
            count
        }
    }
}

pub struct Input {
    stones: Vec<u64>,
}

impl PuzzleInput for Input {
    fn parse(input: &str) -> Self {
        let stones = input
            .replace("\n", "")
            .split_ascii_whitespace()
            .map(|l| l.parse::<u64>().expect("Not a number"))
            .collect();

        Input { stones }
    }
}

const TEST_INPUT: &'static str = "125 17";

#[test]
pub fn test_day_11_part_1() {
    let input = Input::parse(&TEST_INPUT);
    let day = Day11::new();

    assert_eq!(55312, day.solve_part_one(&input));
}
