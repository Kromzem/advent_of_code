pub fn solve(input: &str) -> u32 {
    let mut number_rows: Vec<Vec<Number>> = Vec::new();
    let mut symbol_rows: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let (numbers, symbols) = find_numbers_and_symbols(line);

        number_rows.push(numbers);
        symbol_rows.push(symbols);
    }

    let mut found_numbers: Vec<u32> = Vec::new();
    let mut sum = 0u32;
    for (i, symbol_row) in symbol_rows.iter().enumerate() {
        for symbol_index in symbol_row {
            found_numbers.clear();

            for y in i - 1..=i + 1 {
                let number_row_option = number_rows.get_mut(y);
                if number_row_option.is_none() {
                    continue;
                }

                let mut number_row: Vec<&Number> = number_row_option.unwrap().iter().collect();

                for x in symbol_index - 1..=symbol_index + 1 {
                    let found = number_row.iter().enumerate().find_map(|(i, n)| {
                        if n.index_in_range(x) {
                            Some((i, n.value))
                        } else {
                            None
                        }
                    });

                    if let Some((index, value)) = found {
                        number_row.swap_remove(index);

                        found_numbers.push(value);
                    }
                }
            }

            if found_numbers.len() == 2 {
                sum += found_numbers[0] * found_numbers[1];
            }
        }
    }

    sum
}

fn find_numbers_and_symbols(line: &str) -> (Vec<Number>, Vec<usize>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<usize> = Vec::new();

    let mut number = String::new();
    for (index, c) in line.char_indices() {
        if c.is_digit(10) {
            number.push(c);
            continue;
        }

        if !number.is_empty() {
            numbers.push(Number::new(&number, index - 1));

            let last = numbers.last().unwrap();
            println!(
                "Added number '{}' from {} to {}",
                last.value, last.from_index, last.to_index
            );
            number.clear();
        }

        if c == '.' {
            continue;
        }

        symbols.push(index);
    }

    if !number.is_empty() {
        numbers.push(Number::new(&number, line.len() - 1))
    }

    (numbers, symbols)
}

struct Number {
    value: u32,
    from_index: usize,
    to_index: usize,
}

impl Number {
    fn new(raw: &str, index: usize) -> Number {
        Number {
            value: u32::from_str_radix(raw, 10).expect("This should only contain digits."),
            from_index: index - (raw.len() - 1),
            to_index: index,
        }
    }

    fn index_in_range(&self, index: usize) -> bool {
        index >= self.from_index && index <= self.to_index
    }
}
