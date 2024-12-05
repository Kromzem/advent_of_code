use std::{cmp::Ordering, collections::HashMap};

use crate::Puzzle;

type RuleMap = HashMap<usize, Vec<Rule>>;
type WeightMap = HashMap<(usize, usize), Ordering>;

pub struct Day5 {}

impl Puzzle for Day5 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        5
    }

    fn new() -> Self {
        Day5 {}
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

impl Day5 {
    fn get_input(&self) -> Input {
        let input = self.load_input();

        Input::parse(&input)
    }

    fn solve_part_one_with_input(&self, input: Input) -> i64 {
        input
            .update_sequences
            .into_iter()
            .filter(|s| self.is_sequence_valid(&s, &input.rules))
            .map(|s| s.values[s.values.len() / 2] as i64)
            .sum()
    }

    fn solve_part_two_with_input(&self, input: Input) -> i64 {
        input
            .update_sequences
            .into_iter()
            .filter(|s| !self.is_sequence_valid(&s, &input.rules))
            .map(|s| {
                let correct = self.correct_sequence(&s.values, &input.weight_map);

                correct[correct.len() / 2] as i64
            })
            .sum()
    }

    fn is_sequence_valid(&self, sequence: &UpdateSequence, rule_map: &RuleMap) -> bool {
        for (i, update) in sequence.values.iter().enumerate() {
            let Some(rules) = rule_map.get(update) else {
                continue;
            };

            for rule in rules.iter() {
                match rule {
                    Rule::Before(x) => {
                        let Some(index) = sequence.index_map.get(x) else {
                            continue;
                        };

                        if i > *index {
                            return false;
                        }
                    }
                    Rule::After(x) => {
                        let Some(index) = sequence.index_map.get(x) else {
                            continue;
                        };

                        if i < *index {
                            return false;
                        }
                    }
                }
            }
        }

        return true;
    }

    fn correct_sequence(&self, sequence: &Vec<usize>, weight_map: &WeightMap) -> Vec<usize> {
        let mut to_sort = sequence.clone();

        to_sort.sort_unstable_by(|a, b| *weight_map.get(&(*a, *b)).unwrap_or(&Ordering::Equal));
        to_sort
    }
}

struct Input {
    rules: RuleMap,
    update_sequences: Vec<UpdateSequence>,
    weight_map: WeightMap,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let mut rules = HashMap::new();
        let mut weight_map = HashMap::new();
        loop {
            let line = lines.next().expect("Not enough lines");
            if line.is_empty() {
                break;
            }

            let (first, second) = line.split_once('|').expect("Invalid rule format");
            let (first_num, second_num) = (
                first.parse().expect("Invalid rule number"),
                second.parse().expect("Invalid rule number"),
            );

            let first_entry = rules.entry(first_num).or_insert(Vec::new());
            first_entry.push(Rule::Before(second_num));
            weight_map.insert((first_num, second_num), Ordering::Less);

            let second_entry = rules.entry(second_num).or_insert(Vec::new());
            second_entry.push(Rule::After(first_num));
            weight_map.insert((second_num, first_num), Ordering::Greater);
        }

        let mut update_sequences = Vec::new();
        loop {
            let Some(line) = lines.next() else {
                break;
            };

            update_sequences.push(UpdateSequence::new(
                line.split(',')
                    .map(|n| n.parse().expect("Invalid update number"))
                    .collect(),
            ));
        }

        Input {
            rules,
            update_sequences,
            weight_map,
        }
    }
}

enum Rule {
    Before(usize),
    After(usize),
}

struct UpdateSequence {
    values: Vec<usize>,
    index_map: HashMap<usize, usize>,
}

impl UpdateSequence {
    fn new(values: Vec<usize>) -> Self {
        let index_map = values
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();

        UpdateSequence { values, index_map }
    }
}

#[test]
pub fn test_day5_part_1() {
    const INPUT_STR: &'static str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let day = Day5::new();
    let input = Input::parse(INPUT_STR);

    assert_eq!(143, day.solve_part_one_with_input(input));
}

#[test]
pub fn test_day5_part_2() {
    const INPUT_STR: &'static str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let day = Day5::new();
    let input = Input::parse(INPUT_STR);

    assert_eq!(123, day.solve_part_two_with_input(input));
}
