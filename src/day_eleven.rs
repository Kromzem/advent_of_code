use std::collections::HashSet;

const DRIFT_LENGTH: usize = 1000000;

pub fn solve(input: &str) -> usize {
    let positions = parse_positions(input);

    calc_path_sums(&positions)
}

fn parse_positions(input: &str) -> Vec<(usize, usize)> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut empty_rows = HashSet::new();
    let mut empty_columns = HashSet::new();

    for i in 0..map.len() {
        let row = &map[i];

        if row.iter().any(|&c| c != '.') {
            continue;
        }

        empty_rows.insert(i);
    }

    for i in 0..map[0].len() {
        if map.iter().any(|r| r[i] != '.') {
            continue;
        }

        empty_columns.insert(i);
    }

    let mut positions = Vec::new();
    let mut y_offset = 0;

    for y in 0..map.len() {
        let row = &map[y];

        if empty_rows.contains(&y) {
            y_offset += DRIFT_LENGTH - 1;
        }

        let mut x_offset = 0;
        for x in 0..row.len() {
            if empty_columns.contains(&x) {
                x_offset += DRIFT_LENGTH - 1;
            }

            if row[x] != '#' {
                continue;
            }

            positions.push((x + x_offset, y + y_offset));
        }
    }

    positions
}

fn calc_path_sums(positions: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            sum += dst(positions[i], positions[j]);
        }
    }

    sum
}

fn dst(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    let (x1, y1) = (pos1.0 as i32, pos1.1 as i32);
    let (x2, y2) = (pos2.0 as i32, pos2.1 as i32);

    let solution = (x1 - x2).abs() + (y1 - y2).abs();

    return solution as usize;
}
