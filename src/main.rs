use anyhow::Result;
use std::{fs, time::Instant};
use year_2024::{day_10::Day10, day_8::Day8, day_9::Day9};

use anyhow::bail;
mod year_2024;

fn main() {
    let puzzle = Day10::new();
    let input = puzzle.load_input();
    println!("Process solutions for day {} year {}", puzzle.day(), puzzle.year());
    perform_solution("Part 1", || puzzle.solve_part_one(&input));
    perform_solution("Part 2", || puzzle.solve_part_two(&input));
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

pub trait Puzzle<T>
where
    T: PuzzleInput,
{
    fn year(&self) -> usize;
    fn day(&self) -> usize;

    fn new() -> Self;

    fn load_input(&self) -> T {
        let year = self.year();
        let day = self.day();

        T::parse(
            fs::read_to_string(format!("./inputs/{year}/{day}.txt"))
                .expect("Cannot read input file")
                .as_str(),
        )
    }

    fn solve_part_one(&self, input: &T) -> i64;
    fn solve_part_two(&self, input: &T) -> i64;
}

pub trait PuzzleInput {
    fn parse(input: &str) -> Self;
}

struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone + Copy,
{
    fn new(data: Vec<Vec<T>>) -> Grid<T> {
        Grid { data }
    }

    fn get(&self, x: usize, y: usize) -> Option<T> {
        self.data
            .get(y)
            .and_then(|r| r.get(x))
            .and_then(|c| Some(*c))
    }

    fn insert(&mut self, x: usize, y: usize, value: T) -> Result<()> {
        let Some(row) = self.data.get_mut(y) else {
            bail!("Invalid row")
        };

        if x >= row.len() {
            bail!("Invalid column")
        }

        row[x] = value;

        Ok(())
    }
}
