use miette::Diagnostic;
use ndarray::{Array, Array2, ShapeError};
use std::{fmt::Write, str::FromStr};

#[derive(Debug, Diagnostic, thiserror::Error)]
enum LavaIslandMapError {
    #[error("Tried to parse a pattern with no lines")]
    EmptyPattern,

    #[error(transparent)]
    ArrayShape(#[from] ShapeError),

    #[error("Illegal location character {0}")]
    IllegalLocation(char),
}

#[derive(Debug)]
enum Location {
    Ash,
    Rock,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ash => f.write_char('.'),
            Self::Rock => f.write_char('#'),
        }
    }
}

impl Location {
    const fn from_char(c: char) -> Result<Self, LavaIslandMapError> {
        Ok(match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            c => return Err(LavaIslandMapError::IllegalLocation(c)),
        })
    }
}

#[derive(Debug)]
struct Pattern {
    array: Array2<Location>,
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.array.rows() {
            for location in row {
                location.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Pattern {
    fn new(num_columns: usize, locations: Vec<Location>) -> Result<Self, LavaIslandMapError> {
        debug_assert_eq!(locations.len() % num_columns, 0);
        let num_rows = locations.len() / num_columns;
        let array = Array::from_shape_vec((num_rows, num_columns), locations)?;
        Ok(Self { array })
    }
}

impl FromStr for Pattern {
    type Err = LavaIslandMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_columns = s
            .lines()
            .next()
            .ok_or(LavaIslandMapError::EmptyPattern)?
            .len();
        let locations = s
            .lines()
            .flat_map(|s| s.chars().map(Location::from_char))
            .collect::<Result<Vec<Location>, _>>()?;
        Self::new(num_columns, locations)
    }
}

#[derive(Debug)]
struct LavaIslandMap {
    patterns: Vec<Pattern>,
}

impl FromStr for LavaIslandMap {
    type Err = LavaIslandMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let patterns = s
            .split("\n\n")
            .map(Pattern::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Self { patterns })
    }
}

impl LavaIslandMap {
    fn reflection_positions(&self) -> usize {
        todo!()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_13_test.txt");
    let lava_island_map = LavaIslandMap::from_str(input)?;
    println!("{lava_island_map:#?}");
    let result = lava_island_map.reflection_positions();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() -> Result<(), LavaIslandMapError> {
        let input = include_str!("../inputs/day_11_test.txt");
        let lava_island_map = LavaIslandMap::from_str(input)?; // .unwrap();
        let result = lava_island_map.reflection_positions(); // .unwrap();
        assert_eq!(result, 405);
        Ok(())
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_11.txt");
        let lava_island_map = LavaIslandMap::from_str(input).unwrap();
        let result = lava_island_map.reflection_positions();
        assert_eq!(result, 10_885_634);
    }
}
