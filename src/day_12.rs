pub fn solve(input: &str) -> usize {
    input.lines().map(count_possibilities).sum()
}

fn count_possibilities(line: &str) -> usize {
    // println!("Checking: {}", line);

    let (pattern, target) = line.split_once(' ').unwrap();

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let target_counts: Vec<usize> = target
        .split(',')
        .map(|x| usize::from_str_radix(x, 10).unwrap())
        .collect();
    let possibilities = usize::pow(2, pattern.chars().filter(|&c| c == '?').count() as u32);

    let mut sum = 0;
    for mut i in 0..possibilities {
        let mut group_index = 0;
        let mut count = 0;

        // println!();
        // println!("Try {:b}", i);

        // let mut check = String::new();
        let mut failure = false;
        for j in 0..pattern_chars.len() {
            let mut c = pattern_chars[j];

            if c == '?' {
                if i & 1 != 0 {
                    c = '#';
                } else {
                    c = '.';
                }

                i = i >> 1;
            }

            // check.push(c);

            if c == '#' {
                if group_index >= target_counts.len() {
                    failure = true;
                    break;
                }

                count += 1;
                continue;
            }

            if count == 0 {
                continue;
            }

            // print!("{}, ", count);
            if count != target_counts[group_index] {
                failure = true;
                break;
            }

            group_index += 1;
            count = 0;
        }
        // println!("{}", count);
        // println!("{}", check);

        if failure {
            // println!("Failure");
            continue;
        }

        if count > 0 {
            if count != target_counts[group_index] {
                // println!("Failure");
                continue;
            }

            group_index += 1;
        }

        if group_index == target_counts.len() {
            sum += 1;
            // println!("Success");
        } else {
            // println!("Failure");
        }
    }

    // println!("Possibilities: {}", sum);

    sum
}
