pub fn solve(input: &str) -> u32 {
    let races = parse_races(input);

    races
        .iter()
        .map(|r| r.calc_option_count())
        .fold(1, |a, b| a * b)
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = parse_input_values(lines.next().unwrap());
    let distances = parse_input_values(lines.next().unwrap());

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect()
}

fn parse_input_values(line: &str) -> Vec<u32> {
    line.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect()
}

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn calc_option_count(&self) -> u32 {
        let mut options = 0u32;
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
