use miette::Diagnostic;
use ndarray::{Array, Array2, Axis, ShapeError};
use std::{fmt::Write, str::FromStr};

#[derive(Debug, Diagnostic, thiserror::Error)]
enum PlatformError {
    #[error("Tried to parse a pattern with no lines")]
    EmptyPattern,

    #[error(transparent)]
    ArrayShape(#[from] ShapeError),

    #[error("Illegal location character {0}")]
    IllegalLocation(char),
}

#[derive(Debug, Eq, PartialEq)]
enum Location {
    Round,
    Cube,
    Empty,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Round => f.write_char('O'),
            Self::Cube => f.write_char('#'),
            Self::Empty => f.write_char('.'),
        }
    }
}

impl Location {
    const fn from_char(c: char) -> Result<Self, PlatformError> {
        Ok(match c {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Round,
            c => return Err(PlatformError::IllegalLocation(c)),
        })
    }
}

#[derive(Debug)]
struct Platform {
    array: Array2<Location>,
}

impl std::fmt::Display for Platform {
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

impl Platform {
    fn new(num_columns: usize, locations: Vec<Location>) -> Result<Self, PlatformError> {
        debug_assert_eq!(locations.len() % num_columns, 0);
        let num_rows = locations.len() / num_columns;
        let array = Array::from_shape_vec((num_rows, num_columns), locations)?;
        Ok(Self { array })
    }

    fn total_load(&self) -> usize {
        todo!()
    }
}

impl FromStr for Platform {
    type Err = PlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_columns = s.lines().next().ok_or(PlatformError::EmptyPattern)?.len();
        let locations = s
            .lines()
            .flat_map(|s| s.chars().map(Location::from_char))
            .collect::<Result<Vec<Location>, _>>()?;
        Self::new(num_columns, locations)
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_14_test.txt");
    let platform = Platform::from_str(input)?;
    println!("{platform:#?}");
    let result = platform.total_load();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_14_test_input() -> Result<(), PlatformError> {
        let input = include_str!("../inputs/day_14_test.txt");
        let platform = Platform::from_str(input)?;
        let result = platform.total_load();
        assert_eq!(result, 136);
        Ok(())
    }

    #[test]
    fn check_day_14_full_input() {
        let input = include_str!("../inputs/day_14.txt");
        let platform = Platform::from_str(input).unwrap();
        let result = platform.total_load();
        assert_eq!(result, 27_742);
    }
}
