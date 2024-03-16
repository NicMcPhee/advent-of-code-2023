use std::{ops::Range, str::FromStr};

use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    source: String,
    target: String,
    ranges: Vec<RangeMapping>,
}

#[derive(Debug)]
struct RangeMapping {
    range: Range<u64>,
    offset: i64,
}

impl FromStr for Almanac {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = AlmanacParser::parse(Rule::input, s)?.single()?;
        Ok(AlmanacParser::input(parts).map_err(Into::into)?)
    }
}

impl Almanac {
    fn lowest_location(&self) -> anyhow::Result<u64> {
        todo!()
    }
}

#[derive(Parser)]
#[grammar = "grammars/day_05.pest"]
struct AlmanacParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl AlmanacParser {
    fn input(input: Node) -> Result<Almanac> {
        Ok(match_nodes! { input.into_children();
            [seeds(seeds), map(m)..] => Almanac {
                seeds,
                maps: m.collect(),
            },
        })
    }

    fn seeds(input: Node) -> Result<Vec<u64>> {
        Ok(match_nodes! { input.into_children();
            [number(seed)..] => seed.collect(),
        })
    }

    fn map(input: Node) -> Result<Mapping> {
        Ok(match_nodes! { input.into_children();
            [map_title((source, target)), range_mapping(r)..] => Mapping {
                source,
                target,
                ranges: r.collect(),
            },
        })
    }

    fn map_title(input: Node) -> Result<(String, String)> {
        Ok(match_nodes! { input.into_children();
            [name(source), name(target)] => (source.as_str().to_string(), target.as_str().to_string()),
        })
    }

    fn range_mapping(input: Node) -> Result<RangeMapping> {
        Ok(match_nodes! { input.into_children();
            [number(dest_start), number(source_start), number(length)] => RangeMapping {
                range: source_start..source_start +length,
                offset: dest_start as i64 - source_start as i64,
            },
        })
    }

    fn name(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }

    fn number(input: Node) -> Result<u64> {
        let number = input
            .as_str()
            .parse()
            .expect("All numbers must be a valid unsigned integer.");
        Ok(number)
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/day_05_test.txt");
    let almanac = Almanac::from_str(input)?;
    println!("{almanac:#?}");
    let result = almanac.lowest_location()?;
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod day_04_part_1_tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_05_test.txt");
        let almanac = Almanac::from_str(input).unwrap();
        let result = almanac.lowest_location().unwrap();
        assert_eq!(result, 35);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_05.txt");
        let almanac = Almanac::from_str(input).unwrap();
        let result = almanac.lowest_location().unwrap();
        assert_eq!(result, 6420979);
    }
}
