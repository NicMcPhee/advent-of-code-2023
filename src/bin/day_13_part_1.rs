use miette::Diagnostic;
use ndarray::{Array, Array2, Axis, ShapeError};
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

#[derive(Debug, Eq, PartialEq)]
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

    fn reflection_value(&self) -> Option<usize> {
        // We need to multiply the value returned by `axis_reflection_value`
        // by 100 when it's a horizontal line of reflection. The will happen
        // when we are iterating along the vertical (columns) axis, which is
        // `Axis(1)`. Otherwise we leave the value alone, i.e., multiply by 1.
        [(Axis(0), 1), (Axis(1), 100)]
            .into_iter()
            .find_map(|(a, multiplier)| {
                self.axis_reflection_value(a)
                    .map(|position| multiplier * position)
            })
    }

    fn axis_reflection_value(&self, axis: Axis) -> Option<usize> {
        let num_lanes = self.array.lanes(axis).into_iter().len();
        (1..num_lanes)
            // See if there is a reflection around lane `n`
            // along the given axis. `n` is the number of elements
            // to the left (or above) the lane of reflection.
            .find(|&n| self.check_axis_reflection(axis, n))
    }

    // Look for a lane parallel to the given axis where the pattern is a
    // palindrome on either side of that lane. So if `axis` is `Axis(0)`
    // then we're looking for a horizontal plane of reflection (row), and if
    // `axis` is `Axis(1)` the we're for a vertical plane of reflection (columns).
    fn check_axis_reflection(&self, axis: Axis, n: usize) -> bool {
        let lanes = self.array.lanes(axis);
        lanes
            .clone()
            .into_iter()
            // Get the first `n` lanes
            .take(n)
            // We always want to reverse the first iterator because that ensures
            // that we're checking the palindrome from the inside out.
            .rev()
            // `zip` stops when either iterator returns `None`, so this will only
            // compare the "existing" row pairs and stop as soon as either is empty.
            .zip(lanes.into_iter().skip(n))
            .all(|(r1, r2)| r1 == r2)
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
        self.patterns
            .iter()
            .filter_map(Pattern::reflection_value)
            .sum()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_13.txt");
    let lava_island_map = LavaIslandMap::from_str(input)?;
    // println!("{lava_island_map:#?}");
    let result = lava_island_map.reflection_positions();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_test_input() -> Result<(), LavaIslandMapError> {
        let input = include_str!("../inputs/day_13_test.txt");
        let lava_island_map = LavaIslandMap::from_str(input)?;
        let result = lava_island_map.reflection_positions();
        assert_eq!(result, 405);
        Ok(())
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_13.txt");
        let lava_island_map = LavaIslandMap::from_str(input).unwrap();
        let result = lava_island_map.reflection_positions();
        assert_eq!(result, 27_742);
    }
}
