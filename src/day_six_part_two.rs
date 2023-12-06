pub fn solve(input: &str) -> u64 {
    let race = parse_race(input);

    race.calc_option_count()
}

fn parse_race(input: &str) -> Race {
    let mut lines = input.lines();
    let time = parse_input_value(lines.next().unwrap());
    let distance = parse_input_value(lines.next().unwrap());

    Race { time, distance }
}

fn parse_input_value(line: &str) -> u64 {
    u64::from_str_radix(
        line.split_once(':').unwrap().1.replace(" ", "").as_str(),
        10,
    )
    .unwrap()
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn calc_option_count(&self) -> u64 {
        let mut options = 0u64;
        let target_dist = *&self.distance;
        let time = self.time;

        for i in 0..=time {
            let dist = (time - i) * i;

            if dist > target_dist {
                options += 1;
            }
        }

        options
    }
}
