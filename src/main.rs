// #[macro_use]
// extern crate lazy_static;

// use std::{
//     fs::{self, read_dir, read_to_string, FileTimes},
//     time::Instant,
// };

// use anyhow::{Context, Result};
// use aoc_client::AocClient;
// use ratatui::{DefaultTerminal, Frame};

use std::{fs, time::Instant};

use year_2024::{day_1::Day1, day_2::Day2};

// mod day_12;
// mod day_12_part_2;
// mod day_eight;
// mod day_eight_part_two;
// mod day_eleven;
// mod day_five;
// mod day_five_part_two;
// mod day_four;
// mod day_four_part_two;
// mod day_nine;
// mod day_one;
// mod day_seven;
// mod day_seven_part_two;
// mod day_six;
// mod day_six_part_two;
// mod day_ten;
// mod day_three;
// mod day_three_part_two;
// mod day_two;
// mod day_two_part_two;
mod year_2024;

fn main() {
    // let input = fs::read_to_string("./inputs/day_12.txt").expect("Should read input file");

    let puzzle = Day2::new();
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

// fn main() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     let app_result = App::default().run(&mut terminal);
//     ratatui::restore();
//     app_result
// }

// trait Component {
//     fn update(&mut self, action: Action) -> FollowUpAction;
//     fn render(&self, frame: &mut Frame);
// }

// enum FollowUpAction {
//     None,
//     PromptUserInput,
//     ChangeMainComponent { main_component: Box<dyn Component> },
//     Quit,
// }

// enum Action {
//     KeyPress { key: uint },
//     UserInput { input: String },
// }

// struct App {}

// impl App {
//     fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
//         loop {
//             terminal.draw(|frame| self.draw(frame))?;
//             self.handle_events()?;
//         }
//         Ok(())
//     }

//     fn draw(&self, frame: &mut Frame) {
//         todo!()
//     }

//     fn handle_events(&mut self) -> Result<()> {
//         todo!()
//     }
// }
