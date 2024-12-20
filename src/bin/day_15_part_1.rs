use std::{
    convert::Infallible,
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    str::FromStr,
};

#[derive(Debug)]
struct InitializationSequence {
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Step(String);

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H) {
        u8::hash_slice(self.0.as_bytes(), state);
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
    fn sum_of_hashes(&self) -> u64 {
        let hasher_builder = BuildHasherDefault::<InstructionHasher>::default();

        self.steps
            .iter()
            .map(|step| hasher_builder.hash_one(step))
            .sum()
    }
}

impl FromStr for InitializationSequence {
    type Err = Infallible;

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

fn main() {
    let input = include_str!("../inputs/day_15.txt");
    let init_seq = InitializationSequence::from_str(input).unwrap();
    // println!("{init_seq:#?}");
    let result = init_seq.sum_of_hashes();
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_hash() {
        let hasher_builder = BuildHasherDefault::<InstructionHasher>::default();
        let hash = hasher_builder.hash_one(Step("HASH".to_string()));
        assert_eq!(hash, 52);
    }

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
        assert_eq!(result, 510_792);
    }
}
