use crate::Puzzle;

pub struct Day2 {}

impl Puzzle for Day2 {
    fn year(&self) -> usize {
        2024
    }

    fn day(&self) -> usize {
        2
    }

    fn new() -> Self {
        Day2 {}
    }

    fn solve_part_one(&self) -> i64 {
        let input = self.get_input();

        input.data.into_iter().filter(|d| is_safe(d)).count() as i64
    }

    fn solve_part_two(&self) -> i64 {
        let input = self.get_input();

        input
            .data
            .into_iter()
            .filter(|d| is_safe_with_correction(d.len(), d))
            .count() as i64
    }
}

impl Day2 {
    fn get_input(&self) -> Input {
        let input = self.load_input();

        Input::parse(&input)
    }
}

fn is_safe(numbers: &Vec<i64>) -> bool {
    let mut last_num = numbers[0];
    let dir = i64::signum(numbers[0] - numbers[1]);
    for num in numbers.iter().cloned().skip(1) {
        let dist = last_num - num;

        if i64::signum(dist) != dir {
            return false;
        }

        let abs_dist = i64::abs(dist);
        if abs_dist > 3 || abs_dist < 1 {
            return false;
        }

        last_num = num;
    }

    return true;
}

fn is_safe_with_correction(orig_length: usize, numbers: &Vec<i64>) -> bool {
    let mut last_num = numbers[0];
    let dir = i64::signum(numbers[0] - numbers[1]);
    for num in numbers.iter().cloned().skip(1) {
        let dist = last_num - num;
        let abs_dist = i64::abs(dist);

        if i64::signum(dist) != dir || abs_dist > 3 || abs_dist < 1 {
            if orig_length == numbers.len() {
                for i in 0..orig_length {
                    let mut rem_list = numbers.clone();
                    rem_list.remove(i);

                    if is_safe_with_correction(orig_length, &rem_list) {
                        return true;
                    }
                }
            }

            return false;
        }

        last_num = num;
    }

    return true;
}

struct Input {
    data: Vec<Vec<i64>>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let data = input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().expect("Invalid number"))
                    .collect()
            })
            .collect();

        Input { data }
    }
}
