use std::{iter::repeat, num::ParseIntError, str::FromStr, sync::atomic::AtomicUsize};

use memoize::memoize;
use miette::Diagnostic;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::instrument;

#[derive(Debug, thiserror::Error, Diagnostic)]
enum ConditionRecordsError {
    #[error("No space in one of the rows: {0:#?}")]
    NoSpace(String),
    #[error("Illegal integer count")]
    IllegalCount(#[from] ParseIntError),
    #[error("Illegal character in pattern: {0:#?}")]
    IllegalPatternChar(char),
}

#[derive(Debug, Clone, Copy)]
enum Status {
    Broken,
    Working,
    Unknown,
}

impl TryFrom<char> for Status {
    type Error = ConditionRecordsError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Self::Broken,
            '.' => Self::Working,
            '?' => Self::Unknown,
            _ => return Err(ConditionRecordsError::IllegalPatternChar(value)),
        })
    }
}

#[derive(Debug)]
struct ConditionRecord {
    pattern: Vec<Status>,
    counts: Vec<usize>,
}

impl ConditionRecord {
    #[instrument(ret)]
    fn num_arrangements(&self) -> usize {
        self.count_arrangements(0, 0, 0)
    }

    // #[instrument(ret)]
    // #[memoize]
    fn count_arrangements(
        &self,
        pattern_pos: usize,
        counts_pos: usize,
        broken_count: usize,
    ) -> usize {
        // We've reached the end of the counts, but possibly still have patterns to check.
        // We'll set the current_count (the expected number of broken springs) to 0 since
        // we've exhausted the counts in `self.counts`. If we see any more broken springs,
        // that will cause this branch to "fail" and return 0.
        let current_count = self.counts.get(counts_pos).copied().unwrap_or(0);
        let status = match self.pattern.get(pattern_pos) {
            Some(status) => status,
            // We've exhausted the pattern, the number of broken springs in this block
            // matches the expected number of broken springs, and we're at the last block,
            // we have satisfied the pattern and can return 1.
            None if current_count == broken_count && counts_pos >= self.counts.len() - 1 => {
                return 1;
            }
            // We've exhausted the pattern, and either number of broken springs in this block
            // doesn't match the expected number of broken springs, or we still have additional
            // blocks to satisfy, so we return 0.
            None => return 0,
        };
        let broken_path = match status {
            // Adding this broken spring exceeds the expected number in this group,
            // so this branch "fails" and we return 0.
            Status::Broken | Status::Unknown if broken_count + 1 > current_count => 0,
            Status::Broken | Status::Unknown => {
                self.count_arrangements(pattern_pos + 1, counts_pos, broken_count + 1)
            }
            Status::Working => 0,
        };
        let working_path = match status {
            // If we see a working spring, and the current broken spring count doesn't match
            // the expected broken spring count, then this branch fails and we return 0.
            Status::Working | Status::Unknown
                if broken_count > 0 && broken_count != current_count =>
            {
                0
            }
            Status::Working | Status::Unknown => self.count_arrangements(
                pattern_pos + 1,
                counts_pos + usize::from(broken_count > 0),
                0,
            ),
            Status::Broken => 0,
        };
        broken_path + working_path
    }
}

impl FromStr for ConditionRecord {
    type Err = ConditionRecordsError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (pattern_chars, counts_chars) = line
            .split_once(' ')
            .ok_or_else(|| Self::Err::NoSpace(line.to_string()))?;
        let original_pattern: Vec<Status> = pattern_chars
            .chars()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let repeated_pattern = itertools::Itertools::intersperse(
            repeat(original_pattern).take(5),
            vec![Status::Unknown],
        )
        .flatten()
        .collect();
        let original_counts: Vec<usize> = counts_chars
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let repeated_counts = repeat(original_counts).take(5).flatten().collect();
        Ok(Self {
            pattern: repeated_pattern,
            counts: repeated_counts,
        })
    }
}

#[derive(Debug)]
struct ConditionRecords {
    records: Vec<ConditionRecord>,
}

impl ConditionRecords {
    fn num_arrangements(&self) -> usize {
        let num_completed = AtomicUsize::new(0);
        self.records
            .par_iter()
            .map(|cr| {
                let result = cr.num_arrangements();
                num_completed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                println!("{num_completed:?}/{} => {result}", self.records.len());
                result
            })
            .sum()
    }
}

impl FromIterator<ConditionRecord> for ConditionRecords {
    fn from_iter<T: IntoIterator<Item = ConditionRecord>>(iter: T) -> Self {
        Self {
            records: iter.into_iter().collect(),
        }
    }
}

impl FromStr for ConditionRecords {
    type Err = ConditionRecordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines().map(str::parse).collect()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_12.txt");
    let condition_records: ConditionRecords = input.parse()?;
    // println!("{condition_records:#?}");
    let result = condition_records.num_arrangements();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    #[traced_test]
    #[test_case(". 0", 1 ; "single dot")]
    #[test_case("# 1",  1  ; "single hash")]
    #[test_case("# 1,1", 0 ; "single hash with two counts")]
    #[test_case("? 0", 1 ; "single question mark with zero")]
    #[test_case("? 1", 1 ; "single question mark with one")]
    fn base_cases(input: &'static str, expected: usize) -> Result<(), ConditionRecordsError> {
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, expected);
        Ok(())
    }

    #[traced_test]
    #[test]
    fn check_test_input() -> Result<(), ConditionRecordsError> {
        let input = include_str!("../inputs/day_12_test.txt");
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 21);
        Ok(())
    }

    #[traced_test]
    #[test]
    fn check_full_input() -> Result<(), ConditionRecordsError> {
        let input = include_str!("../inputs/day_12.txt");
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 7718);
        Ok(())
    }
}
