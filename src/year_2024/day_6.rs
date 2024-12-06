use anyhow::Result;
use core::panic;

use anyhow::bail;

use crate::{Grid, Puzzle};

pub struct Day6 {}

impl Puzzle for Day6 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        6
    }

    fn new() -> Self {
        Day6 {}
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

impl Day6 {
    fn get_input(&self) -> Input {
        let input = self.load_input();

        Input::parse(&input)
    }

    fn solve_part_one_with_input(&self, input: Input) -> i64 {
        let mut grid = input.grid;
        let mut guard = input.guard;

        travel_to_end(&mut grid, &mut guard).expect("Loop found");

        grid.data
            .iter()
            .map(|r| r.iter().filter(|t| matches!(t, Tile::Passed(_, _))).count())
            .sum::<usize>() as i64
    }

    fn solve_part_two_with_input(&self, input: Input) -> i64 {
        let mut sum = 0;
        for (y, row) in input.grid.data.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if input.guard.x == x && input.guard.y == y {
                    continue;
                }

                if let Some(Tile::Obstacle) = input.grid.get(x, y) {
                    continue;
                }

                let mut grid = Grid::new(input.grid.data.clone());
                let mut guard = input.guard.clone();
                grid.insert(x, y, Tile::Obstacle).unwrap();

                if let Err(_) = travel_to_end(&mut grid, &mut guard) {
                    sum += 1;
                }
            }
        }

        sum
    }
}

fn travel_to_end(grid: &mut Grid<Tile>, guard: &mut Guard) -> Result<()> {
    loop {
        let (Some(x), Some(y)) = (
            guard.x.checked_add_signed(guard.dir_x),
            guard.y.checked_add_signed(guard.dir_y),
        ) else {
            break;
        };

        let Some(tile) = grid.get(x, y) else {
            break;
        };

        if let Tile::Obstacle = tile {
            if guard.dir_x == 0 {
                guard.dir_x = guard.dir_y * -1;
                guard.dir_y = 0;
            } else {
                guard.dir_y = guard.dir_x;
                guard.dir_x = 0;
            }

            continue;
        }

        if let Tile::Passed(dir_x, dir_y) = tile {
            if dir_x == guard.dir_x && dir_y == guard.dir_y {
                bail!("Loop")
            }
        }

        if let Err(_) = grid.insert(x, y, Tile::Passed(guard.dir_x, guard.dir_y)) {
            break;
        }

        guard.x = x;
        guard.y = y;
    }

    Ok(())
}

struct Input {
    grid: Grid<Tile>,
    guard: Guard,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut guard = None;
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, r)| {
                r.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Tile::Obstacle,
                        '.' => Tile::Free,
                        '^' => {
                            guard = Some(Guard {
                                x,
                                y,
                                dir_x: 0,
                                dir_y: -1,
                            });

                            Tile::Passed(0, -1)
                        }
                        _ => panic!("Invalid map char"),
                    })
                    .collect()
            })
            .collect();

        Input {
            grid: Grid::new(grid),
            guard: guard.expect("No guard found"),
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Obstacle,
    Free,
    Passed(isize, isize),
}

#[derive(Clone, Copy)]
struct Guard {
    x: usize,
    y: usize,
    dir_x: isize,
    dir_y: isize,
}

#[test]
pub fn test_day6_part_1() {
    const INPUT_STR: &'static str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let input = Input::parse(&INPUT_STR);
    let day = Day6::new();

    assert_eq!(41, day.solve_part_one_with_input(input));
}

#[test]
pub fn test_day6_part_2() {
    const INPUT_STR: &'static str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let input = Input::parse(&INPUT_STR);
    let day = Day6::new();

    assert_eq!(6, day.solve_part_two_with_input(input));
}
