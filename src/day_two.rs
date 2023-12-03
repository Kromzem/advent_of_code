use core::panic;

const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

pub fn solve(input: &str) -> u32 {
    input.lines().map(check_game_possible).flatten().sum()
}

fn check_game_possible(line: &str) -> Option<u32> {
    let game_split: Vec<&str> = line.split(':').collect();
    let game_id = u32::from_str_radix(game_split[0].replace("Game ", "").as_str(), 10).unwrap();

    for set in game_split[1].split(';') {
        let grabs = set.trim().split(',');

        for grab in grabs {
            let color_split: Vec<&str> = grab.trim().split_whitespace().collect();

            let amount = u32::from_str_radix(color_split[0], 10).unwrap();

            let max_amount = match color_split[1] {
                "blue" => BLUE_COUNT,
                "red" => RED_COUNT,
                "green" => GREEN_COUNT,
                _ => panic!("Invalid"),
            };

            if amount > max_amount {
                return None;
            }
        }
    }

    Some(game_id)
}
