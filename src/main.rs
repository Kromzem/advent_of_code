use std::{fs, time::Instant};

use year_2024::{day_1::Day1, day_2::Day2, day_3::Day3, day_4::Day4};
mod year_2024;

fn main() {
    let puzzle = Day4::new();
    perform_solution("Part 1", || puzzle.solve_part_one());
    perform_solution("Part 2", || puzzle.solve_part_two());
}

fn perform_solution(name: &str, func: impl Fn() -> i64) {
    let now = Instant::now();
    let solution = func();
    let duration = now.elapsed();

    println!(
        "Solution {} ({}ms): {}",
        name,
        duration.as_millis(),
        solution
    );
}

pub trait Puzzle {
    fn year(&self) -> usize;
    fn day(&self) -> usize;

    fn new() -> Self;

    fn load_input(&self) -> String {
        let year = self.year();
        let day = self.day();

        fs::read_to_string(format!("./inputs/{year}/{day}.txt")).expect("Cannot read input file")
    }

    fn solve_part_one(&self) -> i64;
    fn solve_part_two(&self) -> i64;
}
