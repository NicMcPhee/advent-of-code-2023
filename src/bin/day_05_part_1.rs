use std::{ops::Range, str::FromStr};

use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug)]
enum MappingType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Mapping>,
}

impl Almanac {
    fn convert(&self, value: u64) -> u64 {
        self.maps.iter().fold(value, |acc, m| m.convert(acc))
    }

    fn lowest_location(&self) -> Option<u64> {
        self.seeds.iter().map(|s| self.convert(*s)).min()
    }
}

#[derive(Debug)]
struct Mapping {
    #[allow(dead_code)]
    source: MappingType,
    #[allow(dead_code)]
    target: MappingType,
    ranges: Vec<RangeMapping>,
}

impl Mapping {
    fn convert(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| r.convert(value))
            .unwrap_or(value)
    }
}

#[derive(Debug)]
struct RangeMapping {
    range: Range<u64>,
    offset: i64,
}

impl RangeMapping {
    fn convert(&self, value: u64) -> Option<u64> {
        if !self.range.contains(&value) {
            return None;
        }
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        Some((value as i64 + self.offset) as u64)
    }
}

impl FromStr for Almanac {
    type Err = Error<Rule>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = AlmanacParser::parse(Rule::input, s)?.single()?;
        AlmanacParser::input(parts).map_err(Into::into)
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

    fn map_title(input: Node) -> Result<(MappingType, MappingType)> {
        Ok(match_nodes! { input.into_children();
            [mapping_type(source), mapping_type(target)] => (source, target),
        })
    }

    fn range_mapping(input: Node) -> Result<RangeMapping> {
        Ok(match_nodes! { input.into_children();
            [number(dest_start), number(source_start), number(length)] => RangeMapping {
                range: source_start..source_start +length,
                #[allow(clippy::cast_possible_wrap)]
                offset: dest_start as i64 - source_start as i64,
            },
        })
    }

    fn mapping_type(input: Node) -> Result<MappingType> {
        let str = input.as_str();
        Ok(match str {
            "seed" => MappingType::Seed,
            "soil" => MappingType::Soil,
            "fertilizer" => MappingType::Fertilizer,
            "water" => MappingType::Water,
            "light" => MappingType::Light,
            "temperature" => MappingType::Temperature,
            "humidity" => MappingType::Humidity,
            "location" => MappingType::Location,
            _ => return Err(input.error("Unknown mapping type")),
        })
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
    let input = include_str!("../inputs/day_05.txt");
    let almanac = Almanac::from_str(input)?;
    let result = almanac.lowest_location().expect("No location found");
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod day_05_part_1_tests {
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
        assert_eq!(result, 88_151_870);
    }
}
