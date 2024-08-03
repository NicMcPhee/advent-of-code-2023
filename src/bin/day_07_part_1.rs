use anyhow::Context;
use itertools::Itertools;
use std::str::FromStr;
use strum::FromRepr;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, FromRepr, Hash)]
#[repr(u8)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
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
            'J' => Self::Jack,
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
        let mut counts = cards.iter().counts().into_values().collect::<Vec<_>>();
        counts.sort_unstable();
        match &counts[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!("Illegal hand to classify"),
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
        assert_eq!(result, 6440);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_07.txt");
        let mut game = Game::from_str(input).unwrap();
        let result = game.total_winnings();
        assert_eq!(result, 248_836_197);
    }
}
