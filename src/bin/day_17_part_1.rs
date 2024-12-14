use miette::Diagnostic;
use ndarray::{Array, Array2, ShapeError};
use std::{
    fmt::{Display, Write},
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Diagnostic, thiserror::Error)]
enum ParseError {
    #[error("Tried to parse a pattern with no lines")]
    EmptyPattern,

    #[error(transparent)]
    ArrayShape(#[from] ShapeError),

    #[error("Illegal location character {0}")]
    IllegalLocation(char),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[derive(Debug, Clone, Copy)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl CardinalDirection {
    const fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    const fn rotate_slash(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::North,
            Self::South => Self::West,
            Self::West => Self::South,
        }
    }

    const fn rotate_backslash(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::South,
            Self::South => Self::East,
            Self::West => Self::North,
        }
    }

    const fn split(self) -> [Self; 2] {
        match self {
            Self::East | Self::West => [Self::North, Self::South],
            Self::North | Self::South => [Self::East, Self::West],
        }
    }
}

type Position = (usize, usize);

impl Add<CardinalDirection> for Position {
    type Output = Option<Self>;

    fn add(self, rhs: CardinalDirection) -> Self::Output {
        let (row, col) = self;
        Some(match rhs {
            CardinalDirection::North => (row.checked_sub(1)?, col),
            CardinalDirection::South => (row.checked_add(1)?, col),
            CardinalDirection::East => (row, col.checked_add(1)?),
            CardinalDirection::West => (row, col.checked_sub(1)?),
        })
    }
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "This is not a state machine like Clippy thinks"
)]
#[derive(Debug, Default, Copy, Clone)]
struct EnteredFrom {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl EnteredFrom {
    pub const fn any(self) -> bool {
        self.north || self.south || self.east || self.west
    }
}

    fn left(&self, grid: &Grid) -> Option<(Node, u32)> {
        todo!()


    fn right(&self, grid: &Grid) -> Option<(Node, u32)> {
        todo!()
    }

}
}

#[derive(Debug, Clone)]
struct Grid {
    array: Array2<Location>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.array.rows() {
            for location in row {
                Display::fmt(location, f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(num_columns: usize, locations: Vec<u32>) -> Result<Self, ParseError> {
        debug_assert_eq!(locations.len() % num_columns, 0);
        let num_rows = locations.len() / num_columns;
        let array = Array::from_shape_vec((num_rows, num_columns), locations)?;
        Ok(Self { array })
    }
    }


        let initial_position = (0, 0);
        let (_, cost) = astar::astar(
            &Node::new(initial_position),
            |position| self.successors(position),
            |position| self.manhattan_distance(position),
            |position| self.at_goal(position),
        )?;
        Some(cost)
    }
            .into_iter()
            .flatten()
    }

    fn manhattan_distance(
        &self,
        &Node {
            position: (x, y), ..

    ) -> u32 {
        (x.abs_diff(self.target.0) + y.abs_diff(self.target.1)) as u32
    }

    fn at_goal(&self, &Node { position, .. }: &Node) -> bool {
    }
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_columns = s.lines().next().ok_or(ParseError::EmptyPattern)?.len();
        let locations = s
            .lines()
            .flat_map(str::chars)
            .map(Location::try_from)
            .collect::<Result<Vec<Location>, _>>()?;
        Self::new(num_columns, locations)
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_16.txt");
    let grid = Grid::from_str(input)?;
    // println!("{grid}");
    let result = grid.maximize_energized();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_16_test_input() {
        let input = include_str!("../inputs/day_16_test.txt");
        let grid = Grid::from_str(input).unwrap();
        let result = grid.maximize_energized();
        assert_eq!(result, 51);
    }

    #[test]
    fn check_day_16_full_input() {
        let input = include_str!("../inputs/day_16.txt");
        let grid = Grid::from_str(input).unwrap();
        let result = grid.maximize_energized();
        assert_eq!(result, 7793);
    }
}
