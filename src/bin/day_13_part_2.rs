use miette::Diagnostic;
use ndarray::{indices_of, Array, Array2, Axis, ShapeError};
use std::{collections::HashSet, fmt::Write, str::FromStr};

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

    fn smudge_in_place(&mut self) {
        *self = match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Reflection {
    axis: Axis,
    index: usize,
}

impl Reflection {
    const fn new(axis: Axis, index: usize) -> Self {
        Self { axis, index }
    }

    fn value(&self) -> usize {
        match self.axis {
            Axis(0) => self.index,
            Axis(1) => 100 * self.index,
            axis => unreachable!("Axis {axis:?} should be created"),
        }
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

    fn reflection_value_with_smudges(&mut self) -> Option<usize> {
        let original_reflections = self.reflection_values();
        for index in indices_of(&self.array) {
            let location = self.array.get_mut(index).unwrap();
            location.smudge_in_place();

            let new_reflections = self.reflection_values();

            let location = self.array.get_mut(index).unwrap();
            location.smudge_in_place();

            let mut diff = new_reflections.difference(&original_reflections);
            if let Some(reflection) = diff.next() {
                return Some(reflection.value());
            };
        }
        None
    }

    // Return a `HashSet` of reflections.
    fn reflection_values(&self) -> HashSet<Reflection> {
        // We need to multiply the value returned by `axis_reflection_value`
        // by 100 when it's a horizontal line of reflection. The will happen
        // when we are iterating along the vertical (columns) axis, which is
        // `Axis(1)`. Otherwise we leave the value alone, i.e., multiply by 1.

        [Axis(0), Axis(1)]
            .into_iter()
            .flat_map(|axis| {
                self.axis_reflection_position(axis)
                    .into_iter()
                    .map(move |position| Reflection::new(axis, position))
            })
            .collect()
    }

    fn axis_reflection_position(&self, axis: Axis) -> Vec<usize> {
        let num_lanes = self.array.lanes(axis).into_iter().len();
        (1..num_lanes)
            // See if there is a reflection around lane `n`
            // along the given axis. `n` is the number of elements
            // to the left (or above) the lane of reflection.
            .filter(|&n| self.check_axis_reflection(axis, n))
            .collect()
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
            .all(|(first_lane, second_lane)| first_lane == second_lane)
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
    fn reflection_positions(&mut self) -> usize {
        self.patterns
            .iter_mut()
            .filter_map(Pattern::reflection_value_with_smudges)
            .sum()
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_13.txt");
    let mut lava_island_map = LavaIslandMap::from_str(input)?;
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
        let mut lava_island_map = LavaIslandMap::from_str(input)?;
        let result = lava_island_map.reflection_positions();
        assert_eq!(result, 400);
        Ok(())
    }

    #[test]
    fn check_full_input() {
        let input = include_str!("../inputs/day_13.txt");
        let mut lava_island_map = LavaIslandMap::from_str(input).unwrap();
        let result = lava_island_map.reflection_positions();
        assert_eq!(result, 32_728);
    }
}
