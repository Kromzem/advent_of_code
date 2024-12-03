use regex::Regex;

use crate::Puzzle;

pub struct Day3 {
    regex: Regex,
}

impl Puzzle for Day3 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        3
    }

    fn new() -> Self {
        Day3 {
            regex: Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex"),
        }
    }

    fn solve_part_one(&self) -> i64 {
        let input = self.load_input();

        input.lines().map(|l| self.solve_line(l)).sum()
    }

    fn solve_part_two(&self) -> i64 {
        let input = self.load_input();

        let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
        self.solve_conditional_line(lines.join("").as_str())
    }
}

impl Day3 {
    fn solve_line(&self, line: &str) -> i64 {
        self.regex
            .captures_iter(line)
            .map(|c| {
                let [first, second] = c
                    .extract()
                    .1
                    .map(|n| n.parse::<i64>().expect("Invalid number"));

                first * second
            })
            .sum()
    }

    fn solve_conditional_line(&self, line: &str) -> i64 {
        line.split("do()")
            .map(|p| self.solve_line(p.split("don't()").nth(0).unwrap()))
            .sum()
    }
}
