use std::collections::HashSet;

type Position = (usize, usize);

pub fn solve(input: &str) -> usize {
    let map = Map::from_input(input);

    traverse_paths(&map)
}

fn traverse_paths(map: &Map) -> usize {
    let start_directions = [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ];

    for start_direction in start_directions {
        let mut pos = map.start_pos;
        let mut direction = start_direction;

        let mut bounds: HashSet<(usize, usize)> = HashSet::new();
        bounds.insert(pos.clone());

        let mut count = 0;
        loop {
            let (mut x, mut y) = (pos.0 as i32, pos.1 as i32);

            if let Direction::UP = direction {
                y -= 1;
            } else if let Direction::DOWN = direction {
                y += 1;
            } else if let Direction::LEFT = direction {
                x -= 1;
            } else if let Direction::RIGHT = direction {
                x += 1;
            }

            if x < 0 || y < 0 {
                break;
            }

            pos = (x as usize, y as usize);

            if pos.0 >= map.width || pos.1 >= map.height {
                break;
            }

            count += 1;

            bounds.insert(pos.clone());
            if pos.0 == map.start_pos.0 && pos.1 == map.start_pos.1 {
                //part 1
                // return count / 2 + (count % 2);

                //part 2
                return count_points_in_loop(&map, &bounds);
            }

            let c = map.get_char_at(pos.0, pos.1);
            let new_direction = match (direction, c) {
                (Direction::UP, '|') => Direction::UP,
                (Direction::UP, 'F') => Direction::RIGHT,
                (Direction::UP, '7') => Direction::LEFT,
                (Direction::DOWN, '|') => Direction::DOWN,
                (Direction::DOWN, 'L') => Direction::RIGHT,
                (Direction::DOWN, 'J') => Direction::LEFT,
                (Direction::LEFT, '-') => Direction::LEFT,
                (Direction::LEFT, 'L') => Direction::UP,
                (Direction::LEFT, 'F') => Direction::DOWN,
                (Direction::RIGHT, '-') => Direction::RIGHT,
                (Direction::RIGHT, 'J') => Direction::UP,
                (Direction::RIGHT, '7') => Direction::DOWN,
                _ => Direction::NONE,
            };

            if let Direction::NONE = new_direction {
                break;
            }

            direction = new_direction;
        }
    }

    unreachable!();
}

fn count_points_in_loop(map: &Map, bounds: &HashSet<(usize, usize)>) -> usize {
    let mut count = 0;

    for y in 0..map.height {
        let mut in_bounds = false;
        let mut cached_corner_pipe = None;
        for x in 0..map.width {
            let mut c = map.get_char_at(x, y);

            if c == '.' || !bounds.contains(&(x, y)) {
                if in_bounds {
                    count += 1;
                }

                continue;
            }

            if c == 'S' {
                c = replace_start(map, bounds);
            }

            if c == '-' {
                continue;
            }

            if c == '|' {
                in_bounds = !in_bounds;
                continue;
            }

            if c == 'L' || c == 'F' {
                cached_corner_pipe = Some(c);
                continue;
            }

            let swap = match (c, cached_corner_pipe) {
                ('J', Some('L')) => false,
                ('J', Some('F')) => true,
                ('7', Some('L')) => true,
                ('7', Some('F')) => false,
                _ => unreachable!(),
            };

            cached_corner_pipe = None;
            if swap {
                in_bounds = !in_bounds;
            }
        }
    }

    count
}

fn replace_start(map: &Map, bounds: &HashSet<(usize, usize)>) -> char {
    let (x, y) = map.start_pos;

    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;

    if y > 0 && bounds.contains(&(x, y - 1)) {
        let c = map.get_char_at(x, y - 1);
        up = c == '|' || c == '7' || c == 'F';
    }

    if y < map.height - 1 && bounds.contains(&(x, y + 1)) {
        let c = map.get_char_at(x, y + 1);
        down = c == '|' || c == 'L' || c == 'J';
    }

    if x > 0 && bounds.contains(&(x - 1, y)) {
        let c = map.get_char_at(x - 1, y);
        left = c == '-' || c == 'L' || c == 'F';
    }

    if x < map.width - 1 && bounds.contains(&(x + 1, y)) {
        let c = map.get_char_at(x + 1, y);
        right = c == '-' || c == 'J' || c == '7';
    }

    match (up, down, left, right) {
        (true, true, false, false) => '|',
        (true, false, true, false) => 'J',
        (true, false, false, true) => 'L',
        (false, true, true, false) => '7',
        (false, true, false, true) => 'F',
        (false, false, true, true) => '-',
        _ => unreachable!(),
    }
}

struct Map {
    fields: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start_pos: Position,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let mut fields: Vec<Vec<char>> = Vec::new();
        let mut start_pos = None;

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start_pos = Some((x, y));
                }

                row.push(c);
            }

            fields.push(row);
        }

        let height = fields.len();
        let width = fields.first().unwrap().len();

        Map {
            fields,
            height,
            width,
            start_pos: start_pos.expect("Start point set"),
        }
    }

    fn get_char_at(&self, x: usize, y: usize) -> char {
        self.fields[y][x]
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}
