//thanks to @RansomTime (GitHub) code for my last unsolved issue

use core::panic;
use std::{cmp::Ordering, collections::HashMap};

lazy_static! {
    static ref CARD_VALUES: HashMap<char, u32> = {
        let mut map = HashMap::new();
        map.insert('A', 13);
        map.insert('K', 12);
        map.insert('Q', 11);
        map.insert('T', 10);
        map.insert('9', 9);
        map.insert('8', 8);
        map.insert('7', 7);
        map.insert('6', 6);
        map.insert('5', 5);
        map.insert('4', 4);
        map.insert('3', 3);
        map.insert('2', 2);
        map.insert('J', 1);
        map
    };
}

pub fn solve(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input.lines().map(Hand::parse).collect();
    hands.sort_by(compare_hands);

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| ((i as u64) + 1) * h.bid)
        .sum()
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    let value_cmp = a.value.cmp(&b.value);
    if value_cmp.is_ne() {
        return value_cmp;
    }

    for i in 0..a.cards.len() {
        let a_card_value = CARD_VALUES.get(&a.cards[i]).unwrap();
        let b_card_value = CARD_VALUES.get(&b.cards[i]).unwrap();

        let card_cmp = a_card_value.cmp(b_card_value);
        if card_cmp.is_ne() {
            return card_cmp;
        }
    }

    panic!("Hands must be different!");
}

struct Hand {
    value: u64,
    bid: u64,
    cards: Vec<char>,
}

impl Hand {
    fn parse(line: &str) -> Hand {
        let (cards, bid) = line.split_once(' ').unwrap();

        let card_list: Vec<char> = cards.chars().collect();

        let value = Hand::calc_hand_value(&card_list);

        Hand {
            value,
            bid: u64::from_str_radix(bid, 10).unwrap(),
            cards: card_list,
        }
    }

    fn calc_hand_value(orig_cargs: &Vec<char>) -> u64 {
        let cards: Vec<char> = orig_cargs.iter().cloned().collect();

        let mut card_map = HashMap::new();
        for card in cards {
            *card_map.entry(card).or_insert(0) += 1;
        }

        let joker_count = card_map.remove(&'J').unwrap_or(0);
        let mut values: Vec<i32> = card_map.values().cloned().collect();
        values.sort();
        values.reverse();

        let value = values.first().unwrap_or(&0) + joker_count;
        if value == 5 {
            return 7;
        }

        if value == 4 {
            return 6;
        }

        if value == 3 {
            if values[1] == 2 {
                return 5;
            }

            return 4;
        }

        if value == 2 {
            if values[1] == 2 {
                return 3;
            }

            return 2;
        }

        return 1;
    }
}
