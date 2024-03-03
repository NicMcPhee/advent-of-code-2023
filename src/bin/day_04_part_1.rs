use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug)]
struct ScratchCard {
    // TODO: Convert these to `HashSet` so I can use `intersection` to determine the winning cards easily
    // Alternatively, we could use the `bit-set` crate, which gives
    // use the `BitSet` type, which would work fine here. That would
    // certainly use less storage and probably(?) be faster.
    winning_numbers: Vec<u8>,
    our_numbers: Vec<u8>,
}

impl ScratchCard {
    fn parse_scratchcards(input: &str) -> anyhow::Result<Vec<Self>> {
        let parts = ScratchCardsParser::parse(Rule::input, input)?;
        let parts = parts.single()?;
        ScratchCardsParser::input(parts).map_err(Into::into)
    }

    fn sum_of_values(input: &str) -> anyhow::Result<u32> {
        Ok(Self::parse_scratchcards(input)?
            .iter()
            .map(ScratchCard::value)
            .sum::<u32>())
    }

    fn value(&self) -> u32 {
        todo!()
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

    fn numbers(input: Node) -> Result<Vec<u8>> {
        Ok(match_nodes! { input.into_children();
            [number(n)..] => n.collect(),
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
    let result = ScratchCard::sum_of_values(input)?;
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod day_04_part_1_tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_04_test.txt");
        let result = ScratchCard::sum_of_values(input).unwrap();
        assert_eq!(result, 13);
    }

    // #[test]
    // fn check_full_input() {
    //     let input = include_str!("../inputs/day_04.txt");
    //     let result = ScratchCard::sum_of_values(input).unwrap();
    //     assert_eq!(result, todo!());
    // }
}
