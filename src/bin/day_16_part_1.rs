use miette::Diagnostic;
use ndarray::{Array, Array2, Axis, ShapeError};
use std::{
    collections::HashSet,
    fmt::Write,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Diagnostic, thiserror::Error)]
enum PlatformError {
    #[error("Tried to parse a pattern with no lines")]
    EmptyPattern,

    #[error(transparent)]
    ArrayShape(#[from] ShapeError),

    #[error("Illegal location character {0}")]
    IllegalLocation(char),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
enum Tile {
    Slash,
    Backslash,
    Dash,
    Pipe,
    Empty,
}

struct EnteredFrom {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Index<CardinalDirection> for EnteredFrom {
    type Output = bool;

    fn index(&self, direction: CardinalDirection) -> &Self::Output {
        match direction {
            CardinalDirection::North => &self.north,
            CardinalDirection::South => &self.south,
            CardinalDirection::East => &self.east,
            CardinalDirection::West => &self.west,
        }
    }
}

impl IndexMut<CardinalDirection> for EnteredFrom {
    fn index_mut(&mut self, direction: CardinalDirection) -> &mut Self::Output {
        match direction {
            CardinalDirection::North => &mut self.north,
            CardinalDirection::South => &mut self.south,
            CardinalDirection::East => &mut self.east,
            CardinalDirection::West => &mut self.west,
        }
    }
}

struct Location {
    tile: Tile,
    enteredFrom: EnteredFrom,
    energized: bool,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slash => f.write_char('/'),
            Self::Backslash => f.write_char('\\'),
            Self::Dash => f.write_char('-'),
            Self::Pipe => f.write_char('|'),
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

/// Where we're rolling to.
#[derive(Debug, Clone, Copy)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl CardinalDirection {
    const fn axis(self) -> Axis {
        match self {
            Self::North | Self::South => Axis(0),
            Self::East | Self::West => Axis(1),
        }
    }

    const fn lane_direction(self) -> LaneDirection {
        match self {
            Self::North | Self::West => LaneDirection::Forward,
            Self::South | Self::East => LaneDirection::Reversed,
        }
    }
}

enum LaneDirection {
    Forward,
    Reversed,
}

#[derive(Debug)]
struct Grid {
    array: Array2<Location>,
}

impl std::fmt::Display for Grid {
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

impl Grid {
    fn new(num_columns: usize, locations: Vec<Location>) -> Result<Self, PlatformError> {
        debug_assert_eq!(locations.len() % num_columns, 0);
        let num_rows = locations.len() / num_columns;
        let array = Array::from_shape_vec((num_rows, num_columns), locations)?;
        Ok(Self { array })
    }

    fn num_energized(&self) -> miette::Result<usize> {
        todo!()
    }
}

impl FromStr for Grid {
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
    let input = include_str!("../inputs/day_16_test.txt");
    let grid = Grid::from_str(input)?;
    println!("{grid:#?}");
    let result = grid.num_energized()?;
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_16_test_input() {
        let input = include_str!("../inputs/day_16_test.txt");
        let platform = Grid::from_str(input).unwrap();
        let result = platform.num_energized().unwrap();
        assert_eq!(result, 136);
    }

    #[test]
    fn check_day_16_full_input() {
        let input = include_str!("../inputs/day_16.txt");
        let platform = Grid::from_str(input).unwrap();
        let result = platform.num_energized().unwrap();
        assert_eq!(result, 109_755);
    }
}
