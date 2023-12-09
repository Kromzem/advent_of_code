pub fn solve(input: &str) -> i64 {
    input.lines().map(calc_history_value).sum()
}

fn calc_history_value(line: &str) -> i64 {
    let mut value_rows: Vec<Vec<i64>> = Vec::new();
    value_rows.push(parse_history_value_row(line));

    loop {
        let last = value_rows.last().unwrap();
        if last.iter().all(|&x| x == 0) {
            break;
        }

        value_rows.push(create_row_by_values(last));
    }

    //part 1
    //calc_next_history_value(&value_rows)

    //part 2
    calc_previous_history_value(&value_rows)
}

fn parse_history_value_row(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|x| i64::from_str_radix(x, 10).unwrap())
        .collect()
}

fn create_row_by_values(values: &Vec<i64>) -> Vec<i64> {
    let mut row = Vec::new();

    for i in 1..values.len() {
        row.push(values[i] - values[i - 1]);
    }

    row
}

fn calc_next_history_value(value_rows: &Vec<Vec<i64>>) -> i64 {
    value_rows
        .iter()
        .rev()
        .fold(0, |a, b| a + b.last().unwrap())
}

fn calc_previous_history_value(value_rows: &Vec<Vec<i64>>) -> i64 {
    value_rows.iter().rev().fold(0, |a, b| b[0] - a)
}
