use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    let mut cards: Vec<Scratchcard> = input.lines().map(Scratchcard::new).collect();

    for i in 0..cards.len() {
        let (count, value) = {
            let card = &cards[i];

            (card.count, card.value)
        };

        println!("Card #{}: {} to {} cards", i, count, value);

        cards
            .iter_mut()
            .enumerate()
            .skip(i + 1)
            .take(value)
            .for_each(|c| {
                println!("Card #{}: add {}", c.0, count);

                c.1.won(count)
            });
    }

    cards
        .iter()
        .enumerate()
        .for_each(|(i, c)| println!("Card #{}: {}", i, c.count));

    cards.iter().map(|c| c.count as u32).sum()
}

fn get_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .trim()
        .split_ascii_whitespace()
        .map(|n| u32::from_str_radix(n, 10).expect("Number parsable"))
        .collect()
}

struct Scratchcard {
    value: usize,
    count: usize,
}

impl Scratchcard {
    fn new(data: &str) -> Scratchcard {
        let (_, game) = data.split_once(':').expect("Should be splittable");

        let (winning, users) = game.split_once('|').expect("Two game sites");

        let winning_numbers: HashSet<u32> = HashSet::from_iter(get_numbers(winning));
        let user_numbers = get_numbers(users);

        let count = user_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();

        Scratchcard {
            value: count,
            count: 1,
        }
    }

    fn won(&mut self, amount: usize) {
        println!("Add {} to {}", amount, self.count);
        self.count += amount;
    }
}
