use color_eyre::eyre::Result;
use helpers::InputHelpers;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum CamelCard {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug)]
struct CamelCardHand {
    cards: [CamelCard; 5],
    bid: usize,
}

impl FromStr for CamelCardHand {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (hand_str, bid_str) = s.split_once(' ').unwrap();
        let mut cards = [CamelCard::Two; 5];
        let bid = bid_str.parse().unwrap();
        for (i, c) in hand_str.chars().enumerate() {
            cards[i] = match c {
                '2' => CamelCard::Two,
                '3' => CamelCard::Three,
                '4' => CamelCard::Four,
                '5' => CamelCard::Five,
                '6' => CamelCard::Six,
                '7' => CamelCard::Seven,
                '8' => CamelCard::Eight,
                '9' => CamelCard::Nine,
                'T' => CamelCard::Ten,
                'J' => CamelCard::Jack,
                'Q' => CamelCard::Queen,
                'K' => CamelCard::King,
                'A' => CamelCard::Ace,
                _ => unreachable!("invalid card"),
            }
        }
        Ok(Self { cards, bid })
    }
}

impl CamelCardHand {
    // TTAABBCCDDEE
    fn score(&self, joker_rule: bool) -> usize {
        let mut score = self.type_score(joker_rule);
        for card in self.cards {
            let card_score = if joker_rule && card == CamelCard::Jack {
                1
            } else {
                card as usize
            };
            score = score * 100 + card_score;
        }
        score
    }

    fn type_score(&self, joker_rule: bool) -> usize {
        let mut card_counts = HashMap::new();
        let mut jokers = 0;
        for &card in self.cards.iter() {
            if joker_rule && card == CamelCard::Jack {
                jokers += 1;
                continue;
            }
            *card_counts.entry(card).or_insert(0) += 1;
        }

        let mut pairs = 0;
        let mut threes = 0;
        let mut fours = 0;
        let mut fives = 0;

        for &count in card_counts.values() {
            match count {
                2 => pairs += 1,
                3 => threes += 1,
                4 => fours += 1,
                5 => fives += 1,
                _ => {}
            }
        }

        if jokers > 0 {
            match jokers {
                1 => {
                    if pairs > 0 {
                        pairs -= 1;
                        threes += 1;
                    } else if threes > 0 {
                        threes -= 1;
                        fours += 1;
                    } else if fours > 0 {
                        fours -= 1;
                        fives += 1;
                    } else {
                        pairs += 1;
                    }
                }
                2 => {
                    if pairs > 0 {
                        pairs -= 1;
                        fours += 1;
                    } else if threes > 0 {
                        threes -= 1;
                        fives += 1;
                    } else {
                        threes += 1;
                    }
                }
                3 => {
                    if pairs > 0 {
                        pairs -= 1;
                        fives += 1;
                    } else if threes > 0 {
                        threes -= 1;
                        pairs += 1;
                    } else {
                        fours += 1;
                    }
                }
                4 => return 6,
                5 => return 6,
                _ => unreachable!("invalid joker count"),
            }
        }

        match (pairs, threes, fours, fives) {
            (0, 0, 0, 1) => 6,
            (0, 0, 1, 0) => 5,
            (1, 1, 0, 0) => 4,
            (0, 1, 0, 0) => 3,
            (2, 0, 0, 0) => 2,
            (1, 0, 0, 0) => 1,
            _ => 0,
        }
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let mut hands = input.parse_input::<CamelCardHand>()?;
    hands.sort_by(|a, b| {
        let a_score = a.score(false);
        let b_score = b.score(false);
        a_score.cmp(&b_score)
    });
    let mut i = 1;
    let result = hands.iter().fold(0, |score, card| {
        let score = score + card.bid * i;
        i += 1;
        score
    });
    Ok(result.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let mut hands = input.parse_input::<CamelCardHand>()?;
    hands.sort_by(|a, b| {
        let a_score = a.score(true);
        let b_score = b.score(true);
        a_score.cmp(&b_score)
    });
    let mut i = 1;
    let result = hands.iter().fold(0, |score, card| {
        let score = score + card.bid * i;
        i += 1;
        score
    });
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "6440";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "5905";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn hand_score_test() -> Result<()> {
        let test_hand = CamelCardHand {
            cards: [
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Three,
                CamelCard::King,
            ],
            bid: 123,
        };
        let expected = 31414140313;
        let actual = test_hand.score(false);
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn joker_hands() -> Result<()> {
        let joker_and_high_hand = CamelCardHand {
            cards: [
                CamelCard::Ace,
                CamelCard::King,
                CamelCard::Queen,
                CamelCard::Three,
                CamelCard::Jack,
            ],
            bid: 123,
        };
        let joker_and_high_hand_expected = 1;
        let joker_and_high_hand_actual = joker_and_high_hand.type_score(true);
        assert_eq!(joker_and_high_hand_expected, joker_and_high_hand_actual);

        let joker_and_pair_hand = CamelCardHand {
            cards: [
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Queen,
                CamelCard::Three,
                CamelCard::Jack,
            ],
            bid: 123,
        };

        let joker_and_pair_hand_expected = 3;
        let joker_and_pair_hand_actual = joker_and_pair_hand.type_score(true);
        assert_eq!(joker_and_pair_hand_expected, joker_and_pair_hand_actual);

        let joker_and_3oak_hand = CamelCardHand {
            cards: [
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Three,
                CamelCard::Jack,
            ],
            bid: 123,
        };

        let joker_and_3oak_hand_expected = 5;
        let joker_and_3oak_hand_actual = joker_and_3oak_hand.type_score(true);
        assert_eq!(joker_and_3oak_hand_expected, joker_and_3oak_hand_actual);

        let joker_and_4oak_hand = CamelCardHand {
            cards: [
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Ace,
                CamelCard::Jack,
            ],
            bid: 123,
        };

        let joker_and_4oak_hand_expected = 6;
        let joker_and_4oak_hand_actual = joker_and_4oak_hand.type_score(true);
        assert_eq!(joker_and_4oak_hand_expected, joker_and_4oak_hand_actual);

        Ok(())
    }
}
