use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

struct Card {
    num: u32,
    winners: HashSet<u32>,
    getters: HashSet<u32>,
}

fn calc_winning_points(path: &Path) -> u32 {
    read_to_string(path).unwrap().lines()
        .map(parse_card)
        .map(|c| c.getters.intersection(&c.winners).count() as u32)
        .filter(|n| *n > 0)
        .map(|n| 2u32.pow(n - 1))
        .sum()
}


fn calc_real_winning_points(path: &Path) -> u32 {
    let cards: Vec<Card> = read_to_string(path).unwrap().lines()
        .map(parse_card)
        .collect();
    let mut count = vec![1; cards.len()];
    for i in 0..cards.len() {
        let card = &cards[i];
        let hits = card.getters.intersection(&card.winners).count();
        if hits > 0 {
            let from = i + 1;
            let to = cards.len().min(i + hits);
            for j in from..=to {
                count[j] += count[i];
            }
        }
    }
    let result = count.iter().sum();
    result
}


fn parse_card(line: &str) -> Card {
    let delim = line.find(':').unwrap();
    let num: u32 = line[5..delim].trim().parse().unwrap();
    let splitter = line.find('|').unwrap();
    let winners: HashSet<u32> = line[1 + delim..splitter].split(' ')
        .filter(|d| d.is_empty())
        .map(|d| d.parse::<u32>().unwrap())
        .collect();
    let getters: HashSet<u32> = line[1 + splitter..].split(' ')
        .filter(|d| d.is_empty())
        .map(|d| d.parse::<u32>().unwrap())
        .collect();
    Card {
        num,
        winners,
        getters,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day4/input.txt");
        println!("Day 4, Problem 1: Points: {}", calc_winning_points(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day4/input.txt");
        println!("Day 4, Problem 1: Scratchcards: {}", calc_real_winning_points(test_data.as_path()));
    }
}