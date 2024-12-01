use std::collections::HashMap;

use crate::Puzzle;

pub struct Day1 {}

impl Puzzle for Day1 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        1
    }

    fn solve_part_one(&self) -> i64 {
        let mut input = self.parse_input();

        input.first_list.sort_unstable();
        input.second_list.sort_unstable();

        input
            .first_list
            .iter()
            .zip(input.second_list.iter())
            .map(|(f, s)| i64::abs(f - s))
            .sum()
    }

    fn solve_part_two(&self) -> i64 {
        let input = self.parse_input();

        let mut counts: HashMap<i64, i64> = HashMap::new();
        for number in input.second_list.into_iter() {
            let entry = counts.entry(number).or_insert(0);
            *entry += 1;
        }

        input
            .first_list
            .iter()
            .map(|n| counts.get(n).unwrap_or(&0) * n)
            .sum()
    }

    fn new() -> Self {
        Day1 {}
    }
}

impl Day1 {
    fn parse_input(&self) -> Input {
        let input = self.load_input();

        Input::parse(&input)
    }
}

struct Input {
    first_list: Vec<i64>,
    second_list: Vec<i64>,
}

impl Input {
    fn parse(input: &str) -> Input {
        let (first, second) = input
            .lines()
            .map(|l| l.split_once("   ").expect("Imvalid input"))
            .map(|(f, s)| {
                (
                    f.parse::<i64>().expect("Invalid number"),
                    s.parse::<i64>().expect("Invalid number"),
                )
            })
            .unzip();

        Input {
            first_list: first,
            second_list: second,
        }
    }
}
