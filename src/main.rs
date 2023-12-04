use std::fs;

mod day_four;
mod day_four_part_two;
mod day_one;
mod day_three;
mod day_three_part_two;
mod day_two;
mod day_two_part_two;

fn main() {
    let input = fs::read_to_string("./inputs/day_four.txt").expect("Should read input file");

    let solution = day_four_part_two::solve(&input);

    println!("Solution: {}", solution);
}
