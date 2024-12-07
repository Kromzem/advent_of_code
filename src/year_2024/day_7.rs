use std::collections::HashSet;

use crate::Puzzle;

pub struct Day7 {}

impl Puzzle for Day7 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        7
    }

    fn new() -> Self {
        Day7 {}
    }

    fn solve_part_one(&self) -> i64 {
        let input = self.get_input();

        self.solve_part_one_with_input(input)
    }

    fn solve_part_two(&self) -> i64 {
        let input = self.get_input();

        self.solve_part_two_with_input(input)
    }
}

impl Day7 {
    fn get_input(&self) -> Input {
        let input = self.load_input();

        Input::parse(&input)
    }

    fn solve_part_one_with_input(&self, input: Input) -> i64 {
        let mut possible_ops = HashSet::with_capacity(2);
        possible_ops.insert(Operators::Add);
        possible_ops.insert(Operators::Multiply);

        self.solve(input, possible_ops)
    }

    fn solve_part_two_with_input(&self, input: Input) -> i64 {
        let mut possible_ops = HashSet::with_capacity(2);
        possible_ops.insert(Operators::Add);
        possible_ops.insert(Operators::Multiply);
        possible_ops.insert(Operators::Concat);

        self.solve(input, possible_ops)
    }

    fn solve(&self, input: Input, possible_ops: HashSet<Operators>) -> i64 {
        input
            .equations
            .into_iter()
            .filter(|e| e.can_be_solved(&possible_ops))
            .map(|e| e.solution)
            .sum::<usize>() as i64
    }
}

struct Input {
    equations: Vec<Equation>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let equations = input.lines().map(Equation::parse).collect();

        Input { equations }
    }
}

struct Equation {
    solution: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn parse(to_parse: &str) -> Self {
        let (solution, numbers) = to_parse
            .split_once(':')
            .and_then(|(s, nums)| {
                Some((
                    s.trim().parse().expect("Sokution number invalid"),
                    nums.trim()
                        .split_ascii_whitespace()
                        .map(|n| n.trim().parse().expect("Number invalid"))
                        .collect(),
                ))
            })
            .expect("Cannot parse equation");

        Equation { solution, numbers }
    }

    fn can_be_solved(&self, possible_ops: &HashSet<Operators>) -> bool {
        let mut stop = false;
        let solved = self.check_recursive(self.numbers[0], 1, &mut stop, possible_ops);

        return solved;
    }

    fn check_recursive(
        &self,
        value: usize,
        index: usize,
        stop: &mut bool,
        possible_ops: &HashSet<Operators>,
    ) -> bool {
        if *stop {
            return true;
        }

        if index > self.numbers.len() - 1 {
            if value == self.solution {
                *stop = true;
            }

            return *stop;
        }

        if value > self.solution {
            return false;
        }

        if possible_ops.contains(&Operators::Add) {
            if self.check_recursive(value + self.numbers[index], index + 1, stop, possible_ops) {
                return true;
            }
        }

        if possible_ops.contains(&Operators::Multiply) {
            if self.check_recursive(value * self.numbers[index], index + 1, stop, possible_ops) {
                return true;
            }
        }

        if possible_ops.contains(&Operators::Concat) {
            let Ok(concat_value) = format!("{}{}", value, self.numbers[index]).parse() else {
                return false;
            };

            if self.check_recursive(concat_value, index + 1, stop, possible_ops) {
                return true;
            }
        }

        return false;
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Operators {
    Add,
    Multiply,
    Concat,
}

#[test]
pub fn test_day_7_part_1() {
    const INPUT_STR: &'static str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let input = Input::parse(&INPUT_STR);
    let day = Day7::new();
    assert_eq!(3749, day.solve_part_one_with_input(input))
}

#[test]
pub fn test_day_7_part_2() {
    const INPUT_STR: &'static str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let input = Input::parse(&INPUT_STR);
    let day = Day7::new();
    assert_eq!(11387, day.solve_part_two_with_input(input))
}
