use miette::Diagnostic;
use ndarray::{Array, Array2, ShapeError};
use pathfinding::directed::astar;
use std::{
    fmt::{Display, Write},
    num::NonZeroU8,
    ops::Add,
    str::FromStr,
};

#[derive(Debug, Diagnostic, thiserror::Error)]
enum ParseError {
    #[error("Tried to parse a pattern with no lines")]
    EmptyPattern,

    #[error(transparent)]
    ArrayShape(#[from] ShapeError),

    #[error("An non-digit character {0}")]
    IllegalChar(char),
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("No path was found for this grid")]
struct NoPathFound;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TravelHistory {
    direction: CardinalDirection,
    steps_in_direction: NonZeroU8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: Position,
    travel_history: Option<TravelHistory>,
}

impl Node {
    const fn new(position: Position) -> Self {
        Self {
            position,
            travel_history: None,
        }
    }

    fn left(&self, grid: &Grid) -> Option<(Node, u32)> {
        todo!()
    }

    fn right(&self, grid: &Grid) -> Option<(Node, u32)> {
        todo!()
    }

    fn straight(&self, grid: &Grid) -> Option<(Node, u32)> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Grid {
    array: Array2<u32>,
    target: Position,
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
        let target = (array.ncols() - 1, array.nrows() - 1);
        Ok(Self { array, target })
    }

    fn minimal_heat_loss(&self) -> Option<u32> {
        let initial_position = (0, 0);
        let (_, cost) = astar::astar(
            &Node::new(initial_position),
            |position| self.successors(position),
            |position| self.manhattan_distance(position),
            |position| self.at_goal(position),
        )?;
        Some(cost)
    }

    fn successors(&self, node: &Node) -> impl IntoIterator<Item = (Node, u32)> {
        [node.left(self), node.right(self), node.straight(self)]
            .into_iter()
            .flatten()
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "We know none of this arithmetic will overflow `u32` on the provided inputs"
    )]
    const fn manhattan_distance(
        &self,
        &Node {
            position: (x, y), ..
        }: &Node,
    ) -> u32 {
        (x.abs_diff(self.target.0) + y.abs_diff(self.target.1)) as u32
    }

    fn at_goal(&self, &Node { position, .. }: &Node) -> bool {
        position == self.target
    }
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_columns = s.lines().next().ok_or(ParseError::EmptyPattern)?.len();
        let costs = s
            .lines()
            .flat_map(str::chars)
            .map(|c| c.to_digit(10).ok_or(ParseError::IllegalChar(c)))
            .collect::<Result<Vec<u32>, _>>()?;
        Self::new(num_columns, costs)
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/day_17_test.txt");
    let grid = Grid::from_str(input)?;
    // println!("{grid}");
    let result = grid.minimal_heat_loss().ok_or(NoPathFound)?;
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_17_test_input() {
        let input = include_str!("../inputs/day_17_test.txt");
        let grid = Grid::from_str(input).unwrap();
        let result = grid.minimal_heat_loss().unwrap();
        assert_eq!(result, 102);
    }

    #[test]
    fn check_day_17_full_input() {
        let input = include_str!("../inputs/day_17.txt");
        let grid = Grid::from_str(input).unwrap();
        let result = grid.minimal_heat_loss().unwrap();
        assert_eq!(result, 7793);
    }
}
