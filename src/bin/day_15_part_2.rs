use std::{
    collections::HashMap,
    convert::Infallible,
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    str::FromStr,
};

use strum::FromRepr;

#[derive(Debug)]
struct InitializationSequence {
    steps: Vec<Step>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
enum FocalLength {
    F1 = 1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
}

#[derive(Debug)]
struct Label(Vec<u8>);

impl PartialEq for Label {
    fn eq(&self, _: &Self) -> bool {
        // When used in a `HashMap`, any two `Label`s with the
        // same hash code should be seen as equal. Here we're just
        // saying that _all_ `Label`s are equal, and counting on
        // `HashMap` to discriminate on hash codes
        // before checking equality.
        true
    }
}

impl From<&[u8]> for Label {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}

impl Hash for Label {
    fn hash<H: Hasher>(&self, state: &mut H) {
        u8::hash_slice(&self.0, state);
    }
}

struct Lens {
    label: Label,
    focal_length: FocalLength,
}

#[derive(Debug)]
enum Operation {
    Delete,
    Insert(FocalLength),
}

#[derive(Debug)]
struct Step {
    label: Label,
    op: Operation,
}

#[derive(Debug)]
enum ParseStepError {
    InvalidRepresentation(String),
    IllegalFocalLength(char),
}

impl FromStr for Step {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes() {
            [label @ .., b'=', f] => Self {
                label: label.into(),
                op: Operation::Insert(
                    FocalLength::from_repr(*f - b'0')
                        .ok_or_else(|| ParseStepError::IllegalFocalLength(char::from(*f)))?,
                ),
            },
            [label @ .., b'-'] => Self {
                label: label.into(),
                op: Operation::Delete,
            },
            _ => return Err(ParseStepError::InvalidRepresentation(s.to_string())),
        })
    }
}

#[derive(Default)]
struct InstructionHasher {
    current_value: u8,
}

impl Hasher for InstructionHasher {
    fn finish(&self) -> u64 {
        self.current_value.into()
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            // self.current_value = ((self.current_value + u16::from(*b)) * 17) % 256;
            self.current_value = self.current_value.wrapping_add(*b).wrapping_mul(17);
        }
    }
}

impl InitializationSequence {
    // fn sum_of_hashes(&self) -> u64 {
    //     let hasher_builder = BuildHasherDefault::<InstructionHasher>::default();

    //     self.steps
    //         .iter()
    //         .map(|step| hasher_builder.hash_one(step))
    //         .sum()
    // }

    fn focusing_power(&self) -> u64 {
        let hasher_builder = BuildHasherDefault::<InstructionHasher>::default();
        let boxes = HashMap::<Label, Vec<Lens>, BuildHasherDefault<InstructionHasher>>::with_hasher(
            hasher_builder,
        );

        // Loop over instruction sequence, updating the lenses in the boxes

        // Loop over boxes (using the keys of the `HashMap`)
        //   *Make sure to add one to the box number*
        //   Loop over lens with indices
        //     *Make sure to add one to the index*
        //     Do math
        //   sum()
        // sum()
        todo!()
    }
}

impl FromStr for InitializationSequence {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .trim()
            .split(',')
            .map(Step::from_str)
            .collect::<Result<Vec<_>, ParseStepError>>()?;
        Ok(Self { steps })
    }
}

fn main() {
    let input = include_str!("../inputs/day_15_test.txt");
    let init_seq = InitializationSequence::from_str(input).unwrap();
    // println!("{init_seq:#?}");
    let result = init_seq.focusing_power();
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_15_test_input() {
        let input = include_str!("../inputs/day_15_test.txt");
        let init_seq = InitializationSequence::from_str(input).unwrap();
        let result = init_seq.focusing_power();
        assert_eq!(result, 145);
    }

    #[test]
    fn check_day_15_full_input() {
        let input = include_str!("../inputs/day_15.txt");
        let init_seq = InitializationSequence::from_str(input).unwrap();
        let result = init_seq.focusing_power();
        assert_eq!(result, 510_792);
    }
}
