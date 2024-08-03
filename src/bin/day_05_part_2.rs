use std::{cmp::Ordering, fmt::Display, ops::Range, str::FromStr};

use pest_consume::{match_nodes, Error, Parser};

#[derive(Debug, Copy, Clone)]
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
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
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
    /// so the `target` of one map is the `source` of the next.
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
            mapping.fmt(f)?;
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
    }

    fn lowest_location(&self) -> Option<u64> {
        self.seeds
            .iter()
            .cloned()
            // Convert every seed range to a `RangeMapping`.
            .map(RangeMapping::from_range)
            // Compose each seed `RangeMapping` with the combined mapping. This
            // returns an iterator over all the ranges in the final target type
            // (`location` in this problem). These ranges are the various ranges
            // in the final target space that are reachable from any of the initial
            // seed ranges.
            .flat_map(|mapping| mapping.compose(self.combined_mapping.as_ref().unwrap()))
            // Map each of these reachable ranges to their starting value.
            .map(|r| r.output_range_start())
            // Take the minimum of those values to find the lowest value location.
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

    // Compose two mappings, returning a new mapping that maps from the source
    // space of `self` to the target space of `other`.
    #[allow(clippy::needless_pass_by_value)]
    fn compose(self, other: Self) -> Self {
        let new_ranges = self
            .ranges
            .into_iter()
            // Compose each `RangeMapping` in `self` with `other`.
            // This returns a vector of `RangeMapping`s, so `flat_map`
            // brings all those together into a single `Vec<RangeMapping>`.
            .flat_map(|r| r.compose(&other))
            .collect();
        Self {
            source: self.source,
            target: other.target,
            ranges: new_ranges,
        }
    }

    // Use binary search to find the `RangeMapping` that will map the given
    // `source_index` to a target value.
    fn lookup(&self, source_index: u64) -> Option<&RangeMapping> {
        self.ranges
            .binary_search_by(|r| {
                if source_index < r.range.start {
                    // The range `r` is "greater than" (to the right
                    // of) `source_index.`
                    Ordering::Greater
                } else if r.range.contains(&source_index) {
                    // The range `r` contains `source_index`, so we've
                    // found the desired range.
                    Ordering::Equal
                } else {
                    // The range `r` is "less than" (to the left
                    // of) `source_index`.
                    Ordering::Less
                }
            })
            .ok()
            .and_then(|idx| self.ranges.get(idx))
    }
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
    const fn from_range(range: Range<u64>) -> Self {
        Self { range, offset: 0 }
    }

    const fn output_range_start(&self) -> u64 {
        self.range.start.saturating_add_signed(self.offset)
    }

    // This essentially divides `self` up into a group of contiguous chunks
    // that each map to a different target `RangeMapping` in `other`.
    fn compose(self, other: &Mapping) -> Vec<Self> {
        let mut result = Vec::new();
        // `current_start` is the starting index of the next chunk of
        // `self` that we need to map. That starts at the beginning of
        // `self`.
        let mut current_start = self.range.start;
        // As long as `current_start` is less than `self.range.end`, there's
        // still at least one more non-empty chunk to process.
        while current_start < self.range.end {
            let target_range = other
                // We need to lookup the `RangeMapping` in `other` that the `current_start`
                // would map to after adding the `offset`. Using `saturating_add_signed()`
                // deals with the fact that `current_start` is `u64` and `self.offset` is `i64`,
                // leaving us at `u64::MAX` if for some reason we were to go "off the end".
                .lookup(current_start.saturating_add_signed(self.offset))
                .unwrap_or_else(|| {
                    panic!(
                        "We didn't find a target for {}",
                        current_start.saturating_add_signed(self.offset)
                    )
                });
            // The end of this chunk will be the smaller of the end of `self` (if the remaining
            // bit of `self` is shorter than the `target_range`) and the
            // end of the `target_range`, reverse offset back into the source space
            // (if the `target_range` is shorter than what's left of `self`).
            let current_end = self
                .range
                .end
                .min(target_range.range.end.saturating_add_signed(-self.offset));
            let new_mapping = Self {
                range: current_start..current_end,
                // We can just add the two range offsets to get the combined offset.
                offset: self.offset + target_range.offset,
            };
            result.push(new_mapping);
            current_start = current_end;
        }

        result
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

#[allow(clippy::unnecessary_wraps)]
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
                #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
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
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod day_05_part_2_tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_05_test.txt");
        let almanac = Almanac::from_str(input).unwrap();
        let result = almanac.lowest_location().unwrap();
        assert_eq!(result, 46);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_05.txt");
        let almanac = Almanac::from_str(input).unwrap();
        let result = almanac.lowest_location().unwrap();
        assert_eq!(result, 2_008_785);
    }
}
