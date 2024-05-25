use anyhow::Context;
use itertools::Itertools;
use std::str::FromStr;
use strum::FromRepr;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, FromRepr, Hash)]
#[repr(u8)]
enum Card {
    Joker = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            c @ '2'..='9' => Self::from_repr(c as u8 - b'0').unwrap(),
            'T' => Self::Ten,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => anyhow::bail!("Illegal card character {c}."),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// Deriving `Ord` and `PartialOrd` on the `Hand` struct
// will check the fields from top to bottom. So here
// it will check `HandType` first, using that result
// if it's not `Equal`. If it is `Equal`, then it moves
// on to `cards`, checking them left to right, using
// the ordered provided by the discriminator in the
// enumeration. This is exactly the ordering required
// by the problem, which is quite cool.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self {
            hand_type: Self::classify_hand(&cards),
            cards,
        }
    }

    fn classify_hand(cards: &[Card; 5]) -> HandType {
        let mut counts = cards.iter().counts();
        let num_jokers = counts.remove(&Card::Joker).unwrap_or_default();
        let mut counts = counts.into_values().collect::<Vec<_>>();
        counts.sort_unstable();
        match (&counts[..], num_jokers) {
            ([_], _) | ([], 5) => HandType::FiveOfAKind,
            ([.., x], j) if x+j == 4 => HandType::FourOfAKind,
            ([2, x], j) if x+j == 3 => HandType::FullHouse,
            ([.., x], j) if x+j == 3 => HandType::ThreeOfAKind,
            ([1, 2, 2], 0) => HandType::TwoPair,
            ([.., x], j) if x+j == 2 => HandType::OnePair,
            ([.., 1], 0) => HandType::HighCard,
            _ => unreachable!("Illegal hand to classify {cards:#?} with counts = {counts:#?} and {num_jokers} jokers"),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Round {
    hand: Hand,
    bid: u32,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let (cards, bid) = line
            .split_once(' ')
            .with_context(|| format!("Failed to split the line {line} on whitespace"))?;
        let cards = cards
            .chars()
            .map(Card::try_from)
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self {
            hand: Hand::new(cards.try_into().map_err(|v| {
                anyhow::anyhow!("Failed to convert {v:#?} to an array of 5 `Card`s")
            })?),
            bid: bid.parse()?,
        })
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rounds = s
            .lines()
            .map(Round::from_str)
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self { rounds })
    }
}

impl Game {
    pub fn total_winnings(&mut self) -> u32 {
        self.rounds.sort();
        #[allow(clippy::cast_possible_truncation)]
        self.rounds
            .iter()
            .enumerate()
            .map(|(pos, round)| (pos as u32 + 1) * round.bid)
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_07.txt");
    let mut game = Game::from_str(input)?;
    let result = game.total_winnings();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod day_07_part_1_tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_07_test.txt");
        let mut game = Game::from_str(input).unwrap();
        let result = game.total_winnings();
        assert_eq!(result, 5905);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_07.txt");
        let mut game = Game::from_str(input).unwrap();
        let result = game.total_winnings();
        assert_eq!(result, 251_195_607);
    }
}
