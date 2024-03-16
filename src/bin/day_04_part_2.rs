use std::str::FromStr;

use fixedbitset::FixedBitSet;
use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: FixedBitSet,
    our_numbers: FixedBitSet,
}

impl ScratchCard {
    fn num_winning_numbers(&self) -> usize {
        self.winning_numbers.intersection(&self.our_numbers).count()
    }
}

#[derive(Debug)]
struct ScratchCards {
    cards: Vec<ScratchCard>,
}

impl FromStr for ScratchCards {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = ScratchCardsParser::parse(Rule::input, s)?.single()?;
        Ok(ScratchCards {
            cards: ScratchCardsParser::input(parts).map_err(Into::into)?,
        })
    }
}

impl IntoIterator for ScratchCards {
    type Item = ScratchCard;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl ScratchCards {
    fn num_winning_cards(&self) -> anyhow::Result<usize> {
        let mut counts = vec![1; self.cards.len()];
        for (i, card) in self.cards.iter().enumerate().rev() {
            let num_winning_numbers = card.num_winning_numbers();
            for j in i + 1..=i + num_winning_numbers {
                counts[i] += counts[j];
            }
        }
        Ok(counts.iter().sum())
    }
}

#[derive(Parser)]
#[grammar = "grammars/day_04.pest"]
struct ScratchCardsParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl ScratchCardsParser {
    fn input(input: Node) -> Result<Vec<ScratchCard>> {
        Ok(match_nodes! { input.into_children();
            [scratchcard(c)..] => c.collect::<Vec<ScratchCard>>(),
        })
    }

    fn scratchcard(input: Node) -> Result<ScratchCard> {
        Ok(match_nodes! { input.into_children();
            [number(_), numbers(winning_numbers), numbers(our_numbers)] => ScratchCard {
                winning_numbers,
                our_numbers,
            },
        })
    }

    fn numbers(input: Node) -> Result<FixedBitSet> {
        Ok(match_nodes! { input.into_children();
            [number(n)..] => n.map(Into::into).collect::<FixedBitSet>(),
        })
    }

    fn number(input: Node) -> Result<u8> {
        let number = input
            .as_str()
            .parse()
            .expect("A part number must be a valid unsigned integer.");
        Ok(number)
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_04_test.txt");
    let scratch_cards = ScratchCards::from_str(input)?;
    let result = scratch_cards.num_winning_cards()?;
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod day_04_part_1_tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_04_test.txt");
        let scratch_cards = ScratchCards::from_str(input).unwrap();
        let result = scratch_cards.num_winning_cards().unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_04.txt");
        let scratch_cards = ScratchCards::from_str(input).unwrap();
        let result = scratch_cards.num_winning_cards().unwrap();
        assert_eq!(result, 6420979);
    }
}
