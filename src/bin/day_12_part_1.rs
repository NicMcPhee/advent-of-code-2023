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

#[derive(Debug)]
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
    counts: Vec<i32>,
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
        let counts: Vec<i32> = counts_chars
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
        todo!()
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
    fn check_test_input() -> Result<(), ConditionRecordsError> {
        let input = include_str!("../inputs/day_12_test.txt");
        let condition_records: ConditionRecords = input.parse()?; // .unwrap();
        let result = condition_records.num_arrangements(); // .unwrap();
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
