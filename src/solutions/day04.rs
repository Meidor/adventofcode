use std::{collections::BTreeMap, str::FromStr};

use color_eyre::eyre::Result;
use regex::Regex;

use crate::helpers::InputHelpers;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Card[ ]*([0-9]*): ([0-9 ]*) \| ([0-9 ]*)$").unwrap();
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    yours: Vec<u32>,
}

impl Card {
    fn count_winning_numbers(&self) -> usize {
        self.yours
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cap = RE.captures(s).unwrap();
        let id = cap[1].parse::<u32>()?;
        let winning = cap[2]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let yours = cap[3]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Ok(Card { id, winning, yours })
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    Ok(input
        .parse_input::<Card>()?
        .iter()
        .filter_map(|c| {
            let count = c.count_winning_numbers() as u32;
            (count > 0).then(|| 2u32.pow(count - 1))
        })
        .sum::<u32>()
        .to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let cards = input.parse_input::<Card>()?;
    let card_wins = cards
        .iter()
        .filter_map(|c| {
            let wc = c.count_winning_numbers();
            if wc > 0 {
                Some((c.id, wc))
            } else {
                None
            }
        })
        .collect::<BTreeMap<u32, usize>>();
    let mut card_counts = cards
        .iter()
        .map(|c| (c.id, 1))
        .collect::<BTreeMap<u32, usize>>();
    let total = card_wins.iter().fold(cards.len(), |total, (id, wins)| {
        let amount = *card_counts.get(id).unwrap();
        for i in id + 1..=*id + *wins as u32 {
            *card_counts.entry(i).or_insert(0) += amount;
        }
        total + wins * amount
    });
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "13";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "30";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
