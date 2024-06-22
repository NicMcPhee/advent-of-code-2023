use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;
use miette::Diagnostic;

struct ValueHistory(Vec<i64>);

impl ValueHistory {
    fn predict(&self) -> i64 {
        let first_value = *self.0.first().unwrap();
        if self.0.iter().all_equal() {
            return first_value;
        }
        let predicted_offset = Self(
            self.0
                .iter()
                .tuple_windows()
                .map(|(x, y)| y - x)
                .collect::<Vec<_>>(),
        )
        .predict();
        first_value - predicted_offset
    }
}

#[derive(thiserror::Error, Debug, Diagnostic)]
enum ValueHistoryParseError {
    #[error("Error parsing an integer")]
    ParseInt(#[from] ParseIntError),
}

impl FromStr for ValueHistory {
    type Err = ValueHistoryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i64> = s
            .split_ascii_whitespace()
            .map(i64::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(values))
    }
}

struct Report {
    histories: Vec<ValueHistory>,
}

#[derive(thiserror::Error, Debug, Diagnostic)]
enum ReportParseError {
    #[error("Error parsing a line")]
    #[diagnostic(transparent)]
    ValueHistory(#[from] ValueHistoryParseError),
}

impl FromStr for Report {
    type Err = ReportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let histories = s
            .lines()
            .map(ValueHistory::from_str)
            .collect::<Result<Vec<_>, ValueHistoryParseError>>()?;
        Ok(Self { histories })
    }
}

impl Report {
    fn predictions_total(&self) -> i64 {
        self.histories.iter().map(ValueHistory::predict).sum()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_09.txt");
    let report = Report::from_str(input)?;
    let result = report.predictions_total();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() {
        let input = include_str!("../inputs/day_09_test.txt");
        let report = Report::from_str(input).unwrap();
        let result = report.predictions_total();
        assert_eq!(result, 2);
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_09.txt");
        let report = Report::from_str(input).unwrap();
        let result = report.predictions_total();
        assert_eq!(result, 923);
    }
}
