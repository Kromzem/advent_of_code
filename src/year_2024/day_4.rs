use crate::{Grid, Puzzle};

pub struct Day4 {
    dirs: Vec<(i8, i8)>,
    sequence: Vec<char>,
}

impl Puzzle for Day4 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        4
    }

    fn new() -> Self {
        let dirs = (-1..=1)
            .flat_map(|x| (-1..=1).map(|y| (x, y)).collect::<Vec<(i8, i8)>>())
            .filter(|&(x, y)| x != 0 || y != 0)
            .collect();

        Day4 {
            dirs,
            sequence: vec!['M', 'A', 'S'],
        }
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

impl Day4 {
    fn get_input(&self) -> Input {
        let input = self.load_input();

        Input::parse(&input)
    }

    fn solve_part_one_with_input(&self, input: Input) -> i64 {
        let mut sum = 0;
        for (y, row) in input.grid.data.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c != 'X' {
                    continue;
                }

                sum += self.count_for_x(Coord { x, y }, &input.grid);
            }
        }

        sum
    }

    fn solve_part_two_with_input(&self, input: Input) -> i64 {
        let mut sum = 0;
        for (y, row) in input.grid.data.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c != 'A' {
                    continue;
                }

                if self.check_for_a(Coord { x, y }, &input.grid) {
                    sum += 1;
                }
            }
        }

        sum
    }

    fn count_for_x(&self, from_coord: Coord, grid: &Grid<char>) -> i64 {
        self.dirs
            .iter()
            .filter(|d| self.check_dir(**d, from_coord, grid))
            .count() as i64
    }

    fn check_dir(&self, dir: (i8, i8), from_coord: Coord, grid: &Grid<char>) -> bool {
        for (i, c) in self.sequence.iter().enumerate() {
            let Some(x) = from_coord
                .x
                .checked_add_signed(dir.0 as isize * (i as isize + 1))
            else {
                return false;
            };

            let Some(y) = from_coord
                .y
                .checked_add_signed(dir.1 as isize * (i as isize + 1))
            else {
                return false;
            };

            let Some(value) = grid.get(x, y) else {
                return false;
            };

            if value != *c {
                return false;
            }
        }

        return true;
    }

    fn check_for_a(&self, from_coord: Coord, grid: &Grid<char>) -> bool {
        let x = from_coord.x;
        let y = from_coord.y;

        let (left_up, right_up, left_down, right_down) = (
            grid.get(x - 1, y - 1),
            grid.get(x + 1, y - 1),
            grid.get(x - 1, y + 1),
            grid.get(x + 1, y + 1),
        );

        let pairs = vec![(left_up, right_down), (right_up, left_down)];
        for pair in pairs.into_iter() {
            if match pair {
                (Some('M'), Some('S')) => true,
                (Some('S'), Some('M')) => true,
                _ => false,
            } {
                continue;
            }

            return false;
        }

        return true;
    }
}

struct Input {
    grid: Grid<char>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut grid: Vec<Vec<char>> = input
            .lines()
            .map(|l| {
                //surround grid with chars to ease edge handling
                let mut line = String::new();
                line.push_str(".");
                line.push_str(l);
                line.push_str(".");

                line.chars()
                    .map(|c| match c {
                        'X' | 'M' | 'A' | 'S' => c,
                        _ => '.',
                    })
                    .collect()
            })
            .collect();

        let length = grid[0].len();
        grid.insert(0, vec!['.'; length]);
        grid.push(vec!['.'; length]);

        Input {
            grid: Grid::new(grid),
        }
    }
}

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[test]
pub fn test_day4_part_1() {
    const INPUT_STR: &'static str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let input = Input::parse(&INPUT_STR);
    let day = Day4::new();

    assert_eq!(18, day.solve_part_one_with_input(input));
}

#[test]
pub fn test_day4_part_2() {
    const INPUT_STR: &'static str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let input = Input::parse(&INPUT_STR);
    let day = Day4::new();

    assert_eq!(9, day.solve_part_two_with_input(input));
}
