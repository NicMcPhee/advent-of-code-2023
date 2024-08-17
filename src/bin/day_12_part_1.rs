use std::str::FromStr;

use miette::Diagnostic;

#[derive(Debug, thiserror::Error, Diagnostic)]
enum ConditionRecordsError {}

#[derive(Debug)]
struct ConditionRecords {}

impl ConditionRecords {
    fn num_arrangements(&self) -> usize {
        todo!()
    }
}

impl FromStr for ConditionRecords {
    type Err = ConditionRecordsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_11.txt");
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
