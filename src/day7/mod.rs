use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use lazy_static::lazy_static;

fn calc_winnings(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let mut result: Vec<_> = data.lines()
        .map(|l| {
            let mut splits = l.split(' ').filter(|x| !x.is_empty());
            let cards = splits.next().unwrap();
            let bet = splits.next().unwrap().parse::<u32>().unwrap();
            (cards, bet)
        })
        .map(|(a, b)| (categorize(a), relabel_card(a), b))
        .collect();
    result.sort();
    result.iter().map(|(_, _, bet)| bet).enumerate()
        .map(|(idx, bet)| (1 + idx as u32) * bet)
        .sum()
}


fn calc_winnings_new(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let mut result: Vec<_> = data.lines()
        .map(|l| {
            let mut splits = l.split(' ').filter(|x| !x.is_empty());
            let cards = splits.next().unwrap();
            let bet = splits.next().unwrap().parse::<u32>().unwrap();
            (cards, bet)
        })
        .map(|(a, b)| (categorize_new(a), relabel_card_new(a), b))
        .collect();
    result.sort();
    result.iter().map(|(_, _, bet)| bet).enumerate()
        .map(|(idx, bet)| (1 + idx as u32) * bet)
        .sum()
}

lazy_static! {
static ref CARD_ORDER: HashMap<char, u32> = HashMap::from([
('A', 14), ('K', 13), ('Q', 12),
('J', 11), ('T', 10), ('9', 9),
('8', 8), ('7', 7), ('6', 6),
('5', 5), ('4', 4), ('3', 3),
('2', 2)]);

static ref NEW_CARD_ORDER: HashMap<char, u32> = HashMap::from([
('A', 14), ('K', 13), ('Q', 12),
('T', 10), ('9', 9), ('8', 8),
('7', 7), ('6', 6), ('5', 5),
('4', 4), ('3', 3), ('2', 2),
('J', 1), ]);
}

fn relabel_card(card: &str) -> Vec<u32> {
    card.chars().map(|c| CARD_ORDER.get(&c).unwrap()).copied().collect()
}

fn relabel_card_new(card: &str) -> Vec<u32> {
    card.chars().map(|c| NEW_CARD_ORDER.get(&c).unwrap()).copied().collect()
}


fn categorize_new(cards: &str) -> u32 {
    let mut map = HashMap::new();
    for c in cards.chars().filter(|c| *c != 'J') {
        map.entry(c).and_modify(|d| *d += 1u32).or_insert(1);
    }
    let jokers = cards.chars().filter(|c| *c == 'J').count() as u32;
    let mut val: Vec<u32> = map.values().copied().collect();
    val.sort_by(|a, b| b.cmp(a));
    if jokers == 5 || val[0] + jokers >= 5 {
        return 6;
    }
    if val[0] + jokers == 4 {
        return 5;
    }
    if val[0] + val[1] + jokers >= 5 {
        return 4;
    }
    if val[0] + jokers >= 3 {
        return 3;
    }
    if val[0] + val[1] + jokers >= 4 {
        return 2;
    }
    if val[0] + jokers >= 2 {
        return 1;
    }
    assert_eq!(5, val.len());
    0
}


fn categorize(cards: &str) -> u32 {
    let mut map = HashMap::new();
    for c in cards.chars() {
        map.entry(c).and_modify(|d| *d += 1u32).or_insert(1);
    }
    let mut val: Vec<u32> = map.values().copied().collect();
    val.sort_by(|a, b| b.cmp(a));
    if val[0] == 5 {
        return 6;
    }
    if val[0] == 4 {
        return 5;
    }
    if val[0] == 3 && val[1] == 2 {
        return 4;
    }
    if val[0] == 3 {
        return 3;
    }
    if val[0] == 2 && val[1] == 2 {
        return 2;
    }
    if val[0] == 2 && val[1] == 1 {
        return 1;
    }
    assert_eq!(5, val.len());
    0
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::day7::{calc_winnings, calc_winnings_new};

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day7/input.txt");
        println!("Day 7, Problem 1: : Winnings {}", calc_winnings(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day7/input.txt");
        println!("Day 6, Problem 2: Winnings: {}", calc_winnings_new(test_data.as_path()));
    }
}