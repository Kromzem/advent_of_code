use core::panic;

const NUMBERS: [(&'static str, u32); 18] = [
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5),
    ("five", 5),
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
];

pub fn solve(input: &str) -> u32 {
    input.lines().map(solve_line).sum()
}

fn solve_line(line: &str) -> u32 {
    let first = search_first(line);
    let second = search_second(line);

    first * 10 + second
}

fn search_first(line: &str) -> u32 {
    let mut check = String::new();

    for c in line.chars() {
        check.push(c);

        for (number, value) in NUMBERS {
            if check.contains(number) {
                return value;
            }
        }
    }

    panic!("No number found o.O");
}

fn search_second(line: &str) -> u32 {
    let mut check = String::new();

    for c in line.chars().rev() {
        check.insert(0, c);

        for (number, value) in NUMBERS {
            if check.contains(number) {
                return value;
            }
        }
    }

    panic!("No number found o.O");
}
