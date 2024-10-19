use miette::Diagnostic;
use std::str::FromStr;

#[derive(Debug, Diagnostic, thiserror::Error)]
enum InitSeqError {
    // #[error("Tried to parse a pattern with no lines")]
    // EmptyPattern,

    // #[error(transparent)]
    // ArrayShape(#[from] ShapeError),

    // #[error("Illegal location character {0}")]
    // IllegalLocation(char),
}

#[derive(Debug)]
struct InitSeq {
    // Figure out what we're storing.
}

impl InitSeq {
    // fn new(num_columns: usize, locations: Vec<Location>) -> Result<Self, InitSeqError> {
    //     debug_assert_eq!(locations.len() % num_columns, 0);
    //     let num_rows = locations.len() / num_columns;
    //     let array = Array::from_shape_vec((num_rows, num_columns), locations)?;
    //     Ok(Self { array })
    // }

    fn sum_of_hashes(&self) -> Result<usize, InitSeqError> {
        todo!()
    }
}

impl FromStr for InitSeq {
    type Err = InitSeqError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_15_test.txt");
    let init_seq = InitSeq::from_str(input)?;
    println!("{init_seq:#?}");
    let result = init_seq.sum_of_hashes()?;
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_15_test_input() {
        let input = include_str!("../inputs/day_15_test.txt");
        let init_seq = InitSeq::from_str(input).unwrap();
        let result = init_seq.sum_of_hashes().unwrap();
        assert_eq!(result, 1320);
    }

    #[test]
    fn check_day_15_full_input() {
        let input = include_str!("../inputs/day_15.txt");
        let init_seq = InitSeq::from_str(input).unwrap();
        let result = init_seq.sum_of_hashes().unwrap();
        assert_eq!(result, 109_755);
    }
}
