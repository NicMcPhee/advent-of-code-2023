use miette::Diagnostic;
use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

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
struct InitializationSequence {
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step(String);

// TODO: We should use `BuildHasher` along with `Hasher`.
// We can use `BuildHasherDefault<H>` that impls `BuildHasher`
// for any `H: Hasher`. So I think we can impl `Hasher`, and
// then use `BuildHasherDefault` to get a `BuildHasher`. We can
// then re-use that via the `hash_one()` method to hash our
// strings.

struct InstructionHasher {}

impl InstructionHasher {
    pub fn new() -> Self {
        todo!()
    }
}

impl Hasher for InstructionHasher {
    fn finish(&self) -> u64 {
        todo!()
    }

    fn write(&mut self, bytes: &[u8]) {
        todo!()
    }
}

impl InitializationSequence {
    fn sum_of_hashes(&self) -> u64 {
        self.steps
            .iter()
            .map(|Step(instruction)| {
                let mut hasher = InstructionHasher::new();
                instruction.hash(&mut hasher);
                hasher.finish()
            })
            .sum()
    }
}

impl FromStr for InitializationSequence {
    type Err = InitSeqError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .trim()
            .split(',')
            .map(ToOwned::to_owned)
            .map(Step)
            .collect::<Vec<_>>();
        Ok(Self { steps })
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_15_test.txt");
    let init_seq = InitializationSequence::from_str(input)?;
    println!("{init_seq:#?}");
    let result = init_seq.sum_of_hashes();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_15_test_input() {
        let input = include_str!("../inputs/day_15_test.txt");
        let init_seq = InitializationSequence::from_str(input).unwrap();
        let result = init_seq.sum_of_hashes();
        assert_eq!(result, 1320);
    }

    #[test]
    fn check_day_15_full_input() {
        let input = include_str!("../inputs/day_15.txt");
        let init_seq = InitializationSequence::from_str(input).unwrap();
        let result = init_seq.sum_of_hashes();
        assert_eq!(result, 109_755);
    }
}
