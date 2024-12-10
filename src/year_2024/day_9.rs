use std::isize;

use crate::{Puzzle, PuzzleInput};

pub struct Day9 {}

impl Puzzle<Input> for Day9 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        9
    }

    fn new() -> Self {
        Day9 {}
    }

    fn solve_part_one(&self, input: &Input) -> i64 {
        let mut fragmented = blocks_to_bits(&input.blocks);

        let mut sum = 0;
        let mut index = 0;
        loop {
            index += 1;
            if index >= fragmented.len() - 1 {
                break;
            }

            let value = match fragmented[index] {
                Some(v) => v,
                None => {
                    while fragmented.last().expect("Memory is empty").is_none() {
                        fragmented.pop();
                    }

                    fragmented.swap_remove(index);
                    fragmented[index].expect("Swap invalid")
                }
            };

            sum += index * value;
        }

        sum_of_bits(&fragmented) as i64
    }

    fn solve_part_two(&self, input: &Input) -> i64 {
        let mut fragmented_blocks = input.blocks.clone();

        let mut frag_index = fragmented_blocks.len() as usize;
        loop {
            let Some(index) = frag_index.checked_sub(1) else {
                break;
            };
            frag_index = index;

            let fragment = fragmented_blocks[index];
            let Block::Value { length, id: _ } = fragment else {
                continue;
            };

            let Some((i, free_length)) = fragmented_blocks
                .iter()
                .enumerate()
                .take_while(|&(i, _)| i < frag_index)
                .filter_map(|(i, b)| {
                    if let Block::Free { length: l } = b {
                        if length <= *l {
                            Some((i, *l - length))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .nth(0)
            else {
                continue;
            };

            let moved_fragment = fragmented_blocks.remove(index);
            fragmented_blocks.insert(index, Block::Free { length });
            if free_length == 0 {
                fragmented_blocks.remove(i);
            } else {
                fragmented_blocks[i] = Block::Free {
                    length: free_length,
                };
                frag_index += 1;
            }

            fragmented_blocks.insert(i, moved_fragment);
        }

        let bits = blocks_to_bits(&fragmented_blocks);
        sum_of_bits(&bits) as i64
    }
}

fn blocks_to_bits(blocks: &Vec<Block>) -> Vec<Option<usize>> {
    blocks
        .iter()
        .flat_map(|b| {
            let bits: Vec<Option<usize>> = match b {
                Block::Value { id, length } => (0..*length).map(|_| Some(*id)).collect(),
                Block::Free { length } => (0..*length).map(|_| None).collect(),
            };

            bits
        })
        .collect()
}

fn sum_of_bits(bits: &Vec<Option<usize>>) -> usize {
    bits.into_iter()
        .enumerate()
        .filter_map(|(i, v)| match v {
            Some(id) => Some(i * id),
            None => None,
        })
        .sum()
}

fn print_bits(bits: &Vec<Option<usize>>) {
    for bit in bits {
        let c = match bit {
            Some(v) => char::from_digit(*v as u32, 10).unwrap(),
            None => '.',
        };

        eprint!("{c}");
    }
    eprintln!();
}

pub struct Input {
    blocks: Vec<Block>,
}

impl PuzzleInput for Input {
    fn parse(input: &str) -> Self {
        let blocks = input
            .trim()
            .replace("\n", "")
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let length = char::to_digit(c, 10).expect("Char not a digit") as usize;

                if i % 2 == 0 {
                    Block::Value { id: i / 2, length }
                } else {
                    Block::Free { length }
                }
            })
            .collect();

        Input { blocks }
    }
}

#[derive(Clone, Copy)]
enum Block {
    Value { id: usize, length: usize },
    Free { length: usize },
}

const TEST_INPUT: &'static str = "2333133121414131402";

#[test]
pub fn test_day_9_part_1() {
    let day = Day9::new();
    let input = Input::parse(TEST_INPUT);

    assert_eq!(1928, day.solve_part_one(&input))
}

#[test]
pub fn test_day_9_part_2() {
    let day = Day9::new();
    let input = Input::parse(TEST_INPUT);

    assert_eq!(2858, day.solve_part_two(&input))
}
