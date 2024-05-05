use std::{fmt::Display, ops::Range, str::FromStr};

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

impl Display for MappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Seed => "seed",
            Self::Soil => "soil",
            Self::Fertilizer => "fertilizer",
            Self::Water => "water",
            Self::Light => "light",
            Self::Temperature => "temperature",
            Self::Humidity => "humidity",
            Self::Location => "location",
        })
    }
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
    combined_mapping: Option<Mapping>,
}

impl Display for Almanac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("seeds:")?;
        for seed in &self.seeds {
            let range_len = seed.end - seed.start - 1;
            f.write_str(" ")?;
            seed.start.fmt(f)?;
            f.write_str(" ")?;
            range_len.fmt(f)?;
        }
        f.write_str("\n\n")?;

        if let Some(mapping) = &self.combined_mapping {
            mapping.fmt(f)?
        };

        Ok(())
    }
}

impl Almanac {
    fn new(seeds: Vec<Range<u64>>, mut maps: Vec<Mapping>) -> Self {
        maps.iter_mut().for_each(Mapping::sort_and_fill);
        let combined_mapping = maps.into_iter().reduce(Mapping::compose);
        Self {
            seeds,
            combined_mapping,
    }

    fn convert(&self, value: u64) -> u64 {
        self.maps.iter().fold(value, |acc, m| m.convert(acc))
    }

    fn lowest_location(&self) -> Option<u64> {
        self.seeds
            // Parallelizing the processing of the seed ranges speeds things up fairly
            // substantially on my laptop, which has 12 cores. Without parallelization,
            // this took nearly 90 seconds, where with parallelization it took about 10s.
            .par_iter()
            // A reference to a range can't be iterated over, and thus can't be flattened.
            // Cloning converts the references into owned ranges, which can be iterated over.
            // and thus can be flattened in the next step.
            .cloned()
            .flatten()
            // We tried putting the parallelization here using `par_bridge()`, and that
            // really slowed things down, taking over 240 seconds. Putting it here creates
            // all the seed values _before_ the parallelization, which puts the parallelization
            // too late in the process to have the desired effect, and presumably the overhead
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

impl Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(f)?;
        f.write_str("-to-")?;
        self.target.fmt(f)?;
        f.write_str(" map:\n")?;

        for range in &self.ranges {
            let dest_start = i128::from(range.range.start) + i128::from(range.offset);
            dest_start.fmt(f)?;
            f.write_str(" ")?;
            range.range.start.fmt(f)?;
            f.write_str(" ")?;
            let range_len = range.range.end - range.range.start - 1;
            range_len.fmt(f)?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Mapping {
    fn convert(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| r.convert(value))
            .unwrap_or(value)
    }

    fn sort_and_fill(&mut self) {
        self.ranges.sort();
        let original_ranges = std::mem::take(&mut self.ranges);
        let mut expected_start = 0;
        for range_mapping in original_ranges {
            if expected_start < range_mapping.range.start {
                let padding = RangeMapping {
                    range: expected_start..range_mapping.range.start,
                    offset: 0,
                };
                self.ranges.push(padding);
            }
            expected_start = range_mapping.range.end;
            self.ranges.push(range_mapping);
        }
        if expected_start != u64::MAX {
            let padding = RangeMapping {
                range: expected_start..u64::MAX,
                offset: 0,
            };
            self.ranges.push(padding);
        }
    }

    // Compose two mappings, returning a new mapping.
    // fn compose(&self, other: &Mapping) -> Mapping {}
}

#[derive(Debug, PartialEq, Eq)]
struct RangeMapping {
    // The range is the set of values in the source type.
    range: Range<u64>,
    // The offset to the location in the target type.
    offset: i64,
}

impl PartialOrd for RangeMapping {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeMapping {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range.start.cmp(&other.range.start)
    }
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
            [seeds(seeds), map(m)..] => Almanac::new(seeds, m.collect()),
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
