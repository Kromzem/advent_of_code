use std::collections::HashSet;

use crate::{Grid, Puzzle, PuzzleInput};

pub struct Day10 {
    deltas: Vec<(isize, isize)>,
}

impl Puzzle<Input> for Day10 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        10
    }

    fn new() -> Self {
        Day10 {
            deltas: vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        }
    }

    fn solve_part_one(&self, input: &Input) -> i64 {
        let (_, e) = self.solve(input);

        e as i64
    }

    fn solve_part_two(&self, input: &Input) -> i64 {
        let (p, _) = self.solve(input);

        p as i64
    }
}

impl Day10 {
    fn solve(&self, input: &Input) -> (usize, usize) {
        let mut unique_endpoints = HashSet::new();
        let mut paths = 0;
        let mut endpoints = 0;
        for (y, row) in input.grid.data.iter().enumerate() {
            for (x, &height) in row.iter().enumerate() {
                if height != 0 {
                    continue;
                }

                unique_endpoints.clear();
                paths += self.find_unique_trails(x, y, &input.grid, &mut unique_endpoints);
                endpoints += unique_endpoints.len();
            }
        }

        (paths, endpoints)
    }

    fn find_unique_trails(
        &self,
        x: usize,
        y: usize,
        grid: &Grid<isize>,
        end_points: &mut HashSet<(usize, usize)>,
    ) -> usize {
        let Some(current) = grid.get(x, y) else {
            return 0;
        };

        if current == 9 {
            end_points.insert((x, y));

            return 1;
        }

        let mut sum = 0;
        for &(d_x, d_y) in &self.deltas {
            let Some(next_x) = x.checked_add_signed(d_x) else {
                continue;
            };

            let Some(next_y) = y.checked_add_signed(d_y) else {
                continue;
            };

            let Some(next_value) = grid.get(next_x, next_y) else {
                continue;
            };

            if next_value != current + 1 {
                continue;
            }

            sum += self.find_unique_trails(next_x, next_y, grid, end_points);
        }

        sum
    }
}

pub struct Input {
    grid: Grid<isize>,
}

impl PuzzleInput for Input {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|r| {
                r.chars()
                    .map(|c| c.to_digit(10).expect("Invalid digit") as isize)
                    .collect()
            })
            .collect();

        Input {
            grid: Grid { data: grid },
        }
    }
}

const TEST_INPUT: &'static str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
pub fn test_day_10_part_1() {
    let input = Input::parse(&TEST_INPUT);
    let day = Day10::new();

    assert_eq!(36, day.solve_part_one(&input));
}

#[test]
pub fn test_day_10_part_2() {
    let input = Input::parse(&TEST_INPUT);
    let day = Day10::new();

    assert_eq!(81, day.solve_part_two(&input));
}
