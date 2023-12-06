use std::{fs, time::Instant};

mod day_five;
mod day_five_part_two;
mod day_four;
mod day_four_part_two;
mod day_one;
mod day_six;
mod day_six_part_two;
mod day_three;
mod day_three_part_two;
mod day_two;
mod day_two_part_two;

fn main() {
    let input = fs::read_to_string("./inputs/day_six.txt").expect("Should read input file");

    let now = Instant::now();
    let solution = day_six_part_two::solve(&input);
    let duration = now.elapsed();

    println!("Solution ({}ms): {}", duration.as_millis(), solution);
}
