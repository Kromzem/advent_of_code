pub fn solve(input: &str) -> u32 {
    input.lines().map(check_game_possible).sum()
}

fn check_game_possible(line: &str) -> u32 {
    let game_split: Vec<&str> = line.split(':').collect();

    let mut blue_max = 0u32;
    let mut red_max = 0u32;
    let mut green_max = 0u32;

    for set in game_split[1].split(';') {
        let grabs = set.trim().split(',');

        for grab in grabs {
            let color_split: Vec<&str> = grab.trim().split_whitespace().collect();

            let amount = u32::from_str_radix(color_split[0], 10).unwrap();
            let color = color_split[1];

            if color == "blue" && amount > blue_max {
                blue_max = amount;
            } else if color == "red" && amount > red_max {
                red_max = amount;
            } else if color == "green" && amount > green_max {
                green_max = amount;
            }
        }
    }

    red_max * green_max * blue_max
}
