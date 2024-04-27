use std::{ops::Range, str::FromStr};

use pest_consume::{match_nodes, Error, Parser};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

struct UnknownMappingTypeError(String);

impl FromStr for MappingType {
    type Err = UnknownMappingTypeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "seed" => MappingType::Seed,
            "soil" => MappingType::Soil,
            "fertilizer" => MappingType::Fertilizer,
            "water" => MappingType::Water,
            "light" => MappingType::Light,
            "temperature" => MappingType::Temperature,
            "humidity" => MappingType::Humidity,
            "location" => MappingType::Location,
            _ => return Err(UnknownMappingTypeError(s.to_string())),
        })
    }
}

#[derive(Debug)]
struct Almanac {
    /// Each entry in this `Vec` is a range of seed values,
    /// representing all the seeds in the given range.
    seeds: Vec<Range<u64>>,
    /// Each entry in this `Vec` is a mapping from one type
    /// of value to another, e.g., from `seed` to `soil`. For
    /// this to work, the maps have to be in the right order,
    /// so the `target`` of one map is the `source`` of the next.
    /// (We don't currently _check_ this, though, so it's crucial
    /// that this is correct in the parsed input file.)
    maps: Vec<Mapping>,
}

impl Almanac {
    fn convert(&self, value: u64) -> u64 {
        self.maps.iter().fold(value, |acc, m| m.convert(acc))
    }

    fn lowest_location(&self) -> Option<u64> {
        self.seeds
            // Parallelizing the processing of the seed ranges speeds things up nearly
            // substantially on my laptop, which has 12 cores. Without parallelization,
            // this took nearly 90 seconds, where with parallelization it took about 10.
            .par_iter()
            .cloned()
            .flatten()
            // We tried putting the parallelization here using `par_bridge()`, and that
            // really slowed things down, taking over 240 seconds. Putting it here creates
            // all the seed values _before_ the parallelization, which puts the parallelization
            // to late in the process to have the desired effect, and presumably the overhead
            // of creating the seed values is high.
            .map(|s| self.convert(s))
            .min()
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
#[grammar = "grammars/day_05_part_2.pest"]
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

    fn seeds(input: Node) -> Result<Vec<Range<u64>>> {
        Ok(match_nodes! { input.into_children();
            [seed_pair(seed_pairs)..] => seed_pairs.collect(),
        })
    }

    fn seed_pair(input: Node) -> Result<Range<u64>> {
        Ok(match_nodes! { input.into_children();
            [number(seed), number(length)] => seed .. (seed + length),
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
                offset: dest_start as i64 - source_start as i64,
            },
        })
    }

    fn mapping_type(input: Node) -> Result<MappingType> {
        return MappingType::from_str(input.as_str()).map_err(|e| input.error(e.0));
    }

    fn number(input: Node) -> Result<u64> {
        let number = input
            .as_str()
            .parse()
            .expect("All numbers must be a valid unsigned integer.");
        Ok(number)
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../inputs/day_05.txt");
    let almanac = Almanac::from_str(input)?;
    let result = almanac.lowest_location().expect("No location found");
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
        assert_eq!(result, 46);
    }

    #[test]
    #[ignore = "This test takes several minutes to run"]
    fn check_full_input() {
        let input = include_str!("../inputs/day_05.txt");
        let almanac = Almanac::from_str(input).unwrap();
        let result = almanac.lowest_location().unwrap();
        assert_eq!(result, 2008785);
    }
}
