pub fn solve(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let seeds = parse_seeds(lines[0]);
    let converters = parse_converters(lines.into_iter().skip(1).collect());

    seeds
        .iter()
        .map(|(seed, range)| {
            (*seed..(*seed + *range))
                .map(|s| convert_seed(s, &converters))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn parse_seeds(line: &str) -> Vec<(u32, u32)> {
    let seed_values: Vec<u32> = line
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect();

    let mut seed_ranges = Vec::new();
    for i in (0..seed_values.len()).step_by(2) {
        seed_ranges.push((seed_values[i], seed_values[i + 1]));
    }

    seed_ranges
}

fn parse_converters(lines: Vec<&str>) -> Vec<Vec<Converter>> {
    let mut converters = Vec::new();

    let mut converter_group = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            if converter_group.is_empty() {
                continue;
            }

            converters.push(converter_group);
            converter_group = Vec::new();
            continue;
        }

        converter_group.push(Converter::new(line));
    }

    if !converter_group.is_empty() {
        converters.push(converter_group);
    }

    converters
}

fn convert_seed(seed: u32, converters: &Vec<Vec<Converter>>) -> u32 {
    let mut converted = seed;

    for converter_group in converters.iter() {
        for converter in converter_group.iter() {
            let result = converter.try_convert(converted);

            if let Some(value) = result {
                converted = value;
                break;
            }
        }
    }

    converted
}

struct Converter {
    source: u32,
    dest: u32,
    range: u32,
}

impl Converter {
    fn new(data: &str) -> Converter {
        let parts: Vec<u32> = data
            .split_ascii_whitespace()
            .flat_map(|x| u32::from_str_radix(x, 10))
            .collect();

        Converter {
            source: parts[1],
            dest: parts[0],
            range: parts[2],
        }
    }

    fn try_convert(&self, input: u32) -> Option<u32> {
        if input < self.source {
            return None;
        }

        let range = input - &self.source;
        if range > self.range {
            return None;
        }

        return Some(self.dest + range);
    }
}
