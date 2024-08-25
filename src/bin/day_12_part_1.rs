use std::{num::ParseIntError, str::FromStr};

use miette::Diagnostic;

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
    fn num_arrangements(&self) -> usize {
        println!("About to count arrangements for {self:#?}");
        self.count_arrangements(0, 0, 0)
    }

    fn count_arrangements(
        &self,
        pattern_pos: usize,
        counts_pos: usize,
        broken_count: usize,
    ) -> usize {
        println!("{pattern_pos:^3}, {counts_pos:^3}, {broken_count:^3}");
        // We've reached the end of the counts, but possibly still have patterns to check.
        // We'll set the current_count (the expected number of broken springs) to 0 since
        // we've exhausted the counts in `self.counts`. If we see any more broken springs,
        // that will cause this branch to "fail" and return 0.
        let current_count = self.counts.get(counts_pos).copied().unwrap_or(0);
        let status = match self.pattern.get(pattern_pos) {
            Some(status) => status,
            // We've exhausted the pattern, but were expecting at least one more broken spring,
            // so this branch "fails" and we return 0.
            None if current_count > 0 => return dbg!(0),
            // We've exhausted the pattern and aren't expecting any more broken springs, so
            // this part of this branch is successful and we return 1.
            None => return dbg!(1),
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
        println!("{pattern_pos:^3}, {counts_pos:^3}, {broken_count:^3}, -> {broken_path:^3} + {working_path:^3}");
        broken_path + working_path
    }
}

impl FromStr for ConditionRecord {
    type Err = ConditionRecordsError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (pattern_chars, counts_chars) = line
            .split_once(' ')
            .ok_or_else(|| Self::Err::NoSpace(line.to_string()))?;
        let pattern: Vec<Status> = pattern_chars
            .chars()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        let counts: Vec<usize> = counts_chars
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self { pattern, counts })
    }
}

#[derive(Debug)]
struct ConditionRecords {
    records: Vec<ConditionRecord>,
}

impl ConditionRecords {
    fn num_arrangements(&self) -> usize {
        self.records
            .iter()
            .map(ConditionRecord::num_arrangements)
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
    let input = include_str!("../inputs/day_12_test.txt");
    let condition_records: ConditionRecords = input.parse()?;
    println!("{condition_records:#?}");
    let result = condition_records.num_arrangements();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_single_dot() -> Result<(), ConditionRecordsError> {
        let input = ". 0";
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn check_single_hash() -> Result<(), ConditionRecordsError> {
        let input = "# 1";
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn check_single_question_zero() -> Result<(), ConditionRecordsError> {
        let input = "? 0";
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn check_single_question_one() -> Result<(), ConditionRecordsError> {
        let input = "? 1";
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn check_test_input() -> Result<(), ConditionRecordsError> {
        let input = include_str!("../inputs/day_12_test.txt");
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 21);
        Ok(())
    }

    #[test]
    fn check_full_input() -> Result<(), ConditionRecordsError> {
        let input = include_str!("../inputs/day_12.txt");
        let condition_records: ConditionRecords = input.parse()?;
        let result = condition_records.num_arrangements();
        assert_eq!(result, 10_885_634);
        Ok(())
    }
}
