use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut map = HashMap::new();
    while let Some(path) = lines.next() {
        let (name, choices) = path.split_once('=').unwrap();
        let (left, right) = choices[2..choices.len() - 1].split_once(", ").unwrap();

        map.insert(name.trim(), (left, right));
    }

    let mut position = "AAA";
    let mut count = 0;
    let mut index = 0;

    while position != "ZZZ" {
        let (left, right) = map.get(position).unwrap();
        let instruction = instructions[index];

        position = if instruction == 'R' { right } else { left };
        count += 1;
        index += 1;

        if index >= instructions.len() {
            index -= instructions.len();
        }
    }

    count as u32
}
