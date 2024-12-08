use std::collections::{HashMap, HashSet};

use crate::{Puzzle, PuzzleInput};

pub struct Day8 {}

impl Puzzle<Input> for Day8 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        8
    }

    fn new() -> Self {
        Day8 {}
    }

    fn solve_part_one(&self, input: &Input) -> i64 {
        solve(input, false)
    }

    fn solve_part_two(&self, input: &Input) -> i64 {
        solve(input, true)
    }
}

fn solve(input: &Input, traverse: bool) -> i64 {
    let mut unique_antinodes = HashSet::new();
    for antenna_group in input.antennas.values() {
        for (i, antenna) in antenna_group.iter().enumerate() {
            for related_antenna in antenna_group.iter().skip(i + 1) {
                let x_diff = antenna.0 as isize - related_antenna.0 as isize;
                let y_diff = antenna.1 as isize - related_antenna.1 as isize;

                if traverse {
                    unique_antinodes.insert((antenna.0, antenna.1));
                    unique_antinodes.insert((related_antenna.0, related_antenna.1));
                }

                traverse_all_antinodes_in_line(
                    x_diff,
                    y_diff,
                    antenna.0 as isize,
                    antenna.1 as isize,
                    input.map_size.0,
                    input.map_size.1,
                    &mut unique_antinodes,
                    traverse,
                );

                traverse_all_antinodes_in_line(
                    x_diff * -1,
                    y_diff * -1,
                    related_antenna.0 as isize,
                    related_antenna.1 as isize,
                    input.map_size.0,
                    input.map_size.1,
                    &mut unique_antinodes,
                    traverse,
                );
            }
        }
    }

    unique_antinodes.len() as i64
}

fn traverse_all_antinodes_in_line(
    x_dist: isize,
    y_dist: isize,
    curr_x: isize,
    curr_y: isize,
    map_width: usize,
    map_height: usize,
    nodes: &mut HashSet<(usize, usize)>,
    traverse: bool,
) {
    let new_x = curr_x + x_dist;
    let new_y = curr_y + y_dist;

    if new_x < 0 || new_y < 0 {
        return;
    }

    let (new_u_x, new_u_y) = (new_x as usize, new_y as usize);
    if new_u_x >= map_width || new_u_y >= map_height {
        return;
    }

    nodes.insert((new_u_x, new_u_y));

    if !traverse {
        return;
    }

    traverse_all_antinodes_in_line(
        x_dist, y_dist, new_x, new_y, map_width, map_height, nodes, traverse,
    );
}

pub struct Input {
    antennas: HashMap<char, Vec<(usize, usize)>>,
    map_size: (usize, usize),
}

impl PuzzleInput for Input {
    fn parse(input: &str) -> Self {
        let mut antennas = HashMap::new();
        let mut antenna_locations = HashSet::new();
        let map_size = (
            input.lines().nth(0).expect("No lines").len(),
            input.lines().count(),
        );

        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '.' {
                    continue;
                }

                let entry = antennas.entry(c).or_insert(Vec::new());
                entry.push((x, y));
                antenna_locations.insert((x, y));
            }
        }

        Input { antennas, map_size }
    }
}

fn get_test_input() -> Input {
    const TEST_INPUT: &'static str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    Input::parse(&TEST_INPUT)
}

#[test]
pub fn test_day_8_part_1() {
    let input = get_test_input();
    let day = Day8::new();

    assert_eq!(14, day.solve_part_one(&input));
}

#[test]
pub fn test_day_8_part_2() {
    let input = get_test_input();
    let day = Day8::new();

    assert_eq!(34, day.solve_part_two(&input));
}
