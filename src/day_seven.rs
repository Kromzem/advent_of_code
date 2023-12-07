use core::panic;
use std::{
    cmp::Ordering,
    collections::{btree_map::Values, HashMap},
};

lazy_static! {
    static ref CARD_VALUES: HashMap<char, u32> = {
        let mut map = HashMap::new();
        map.insert('A', 13);
        map.insert('K', 12);
        map.insert('Q', 11);
        map.insert('J', 10);
        map.insert('T', 9);
        map.insert('9', 8);
        map.insert('8', 7);
        map.insert('7', 6);
        map.insert('6', 5);
        map.insert('5', 4);
        map.insert('4', 3);
        map.insert('3', 2);
        map.insert('2', 1);
        map
    };
}

pub fn solve(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input.lines().map(Hand::parse).collect();
    hands.sort_unstable_by(compare_hands);

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
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
    value: u32,
    bid: u32,
    cards: Vec<char>,
}

impl Hand {
    fn parse(line: &str) -> Hand {
        let (cards, bid) = line.split_once(' ').unwrap();

        let card_list: Vec<char> = cards.chars().collect();

        let value = Hand::calc_hand_value(&card_list);

        if value == 0 {
            println!("{} has 0 value!", cards);
        }

        Hand {
            value,
            bid: u32::from_str_radix(bid, 10).unwrap(),
            cards: card_list,
        }
    }

    fn calc_hand_value(orig_cargs: &Vec<char>) -> u32 {
        let cards: Vec<char> = orig_cargs.iter().cloned().collect();

        let mut card_map = HashMap::new();
        for card in cards {
            *card_map.entry(card).or_insert(0) += 1;
        }

        let values: Vec<i32> = card_map.values().cloned().collect();
        if values.contains(&5) {
            return 7;
        }

        if values.contains(&4) {
            return 6;
        }

        if values.contains(&3) {
            if values.contains(&2) {
                return 5;
            }

            return 4;
        }

        let pair_count = values.iter().filter(|&&x| x == 2).count();
        if pair_count == 2 {
            return 3;
        }

        if pair_count == 1 {
            return 2;
        }

        return 1;
    }
}
